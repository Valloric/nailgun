// Copyright 2014 Strahinja Val Markovic
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use base::unicode::{bytesFollowing, readCodepoint};
use super::{Expression, ParseState, ParseResult};

macro_rules! class( ( $ex:expr ) => (
      &base::CharClass::new( $ex.as_bytes() ) ) );


fn toU32Vector( input: &[u8] ) -> Vec<u32> {
  let mut i = 0;
  let mut out_vec : Vec<u32> = vec!();
  loop {
    match input.get( i ) {
      Some( byte ) => match bytesFollowing( *byte ) {
        Some( num_following ) => {
          if num_following > 0 {
            match readCodepoint( &input[ i.. ] ) {
              Some( ch ) => {
                out_vec.push( ch as u32 );
                i += num_following + 1
              }
              _ => { out_vec.push( *byte as u32 ); i += 1 }
            };
          } else { out_vec.push( *byte as u32 ); i += 1 }
        }
        _ => { out_vec.push( *byte as u32 ); i += 1 }
      },
      _ => return out_vec
    }
  }
}


pub struct CharClass {
  // All the single chars in the char class.
  // May be unicode codepoints or binary octets stored as codepoints.
  single_chars: Vec<u32>,

  // Sequence of [from, to] (inclusive bounds) char ranges.
  // May be unicode codepoints or binary octets stored as codepoints.
  ranges: Vec<( u32, u32 )>
}


impl CharClass {
  // Takes the inner content of square brackets, so for [a-z], send "a-z".
  pub fn new( contents: &[u8] ) -> CharClass {
    fn rangeAtIndex( index: usize, chars: &[u32] ) -> Option<( u32, u32 )> {
      match ( chars.get( index ),
              chars.get( index + 1 ),
              chars.get( index + 2 ) ) {
        ( Some( char1 ), Some( char2 ), Some( char3 ) )
            if *char2 == '-' as u32 => Some( ( *char1, *char3 ) ),
        _ => None
      }
    }

    let chars = toU32Vector( &contents );
    let mut char_class = CharClass { single_chars: Vec::new(),
                                     ranges: Vec::new() };
    let mut index = 0;
    loop {
      match rangeAtIndex( index, &chars ) {
        Some( range ) => {
          char_class.ranges.push( range );
          index += 3;
        }
        _ => {
          if index >= chars.len() {
            break
          }
          char_class.single_chars.push( chars[ index ] );
          index += 1;
        }
      };
    }

    char_class
  }

  fn matches( &self, character: u32 ) -> bool {
    return self.single_chars.contains( &character ) ||
      self.ranges.iter().any(
        | &(from, to) | character >= from && character <= to );
  }


  fn applyToUtf8<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    match readCodepoint( parse_state.input ) {
      Some( ch ) if self.matches( ch as u32 ) => {
        let num_following = bytesFollowing( parse_state.input[ 0 ] ).unwrap();
        parse_state.offsetToResult( parse_state.offset + num_following + 1 )
      }
      _ => None
    }
  }


  fn applyToBytes<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    match parse_state.input.get( 0 ) {
      Some( byte ) if self.matches( *byte as u32 ) => {
        parse_state.offsetToResult( parse_state.offset + 1 )
      }
      _ => None
    }
  }
}


impl Expression for CharClass {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    self.applyToUtf8( parse_state ).or( self.applyToBytes( parse_state ) )
  }
}


#[cfg(test)]
mod tests {
  use base;
  use base::{Node, Data, ParseResult, Expression, ParseState};
  use base::test_utils::ToParseState;
  use base::unicode::bytesFollowing;
  use super::{CharClass};

  fn charClassMatch( char_class: &Expression, input: &[u8] ) -> bool {
    fn bytesRead( input: &[u8] ) -> usize {
      bytesFollowing( input[ 0 ] ).map_or( 1, |num| num + 1 )
    }

    match char_class.apply( &ToParseState( input ) ) {
      Some( ParseResult { nodes, parse_state } ) => {
        let bytes_read = bytesRead( input );
        assert_eq!( nodes[ 0 ],
                    Node::withoutName( 0, bytes_read, Data( input ) ) );
        assert_eq!( parse_state, ParseState{ input: &[], offset: bytes_read } );
        true
      }
      _ => false
    }
  }


  #[test]
  fn CharClass_Match() {
    assert!( charClassMatch( class!( "a"          ), b"a" ) );
    assert!( charClassMatch( class!( "abcdef"     ), b"e" ) );
    assert!( charClassMatch( class!( "a-z"        ), b"a" ) );
    assert!( charClassMatch( class!( "a-z"        ), b"c" ) );
    assert!( charClassMatch( class!( "a-z"        ), b"z" ) );
    assert!( charClassMatch( class!( "0-9"        ), b"2" ) );
    assert!( charClassMatch( class!( "α-ω"        ), "η".as_bytes() ) );
    assert!( charClassMatch( class!( "-"          ), b"-" ) );
    assert!( charClassMatch( class!( "a-"         ), b"-" ) );
    assert!( charClassMatch( class!( "-a"         ), b"-" ) );
    assert!( charClassMatch( class!( "a-zA-Z-"    ), b"-" ) );
    assert!( charClassMatch( class!( "aa-zA-Z-a"  ), b"-" ) );
    assert!( charClassMatch( class!( "a-zA-Z-"    ), b"z" ) );
    assert!( charClassMatch( class!( "aa-zA-Z-0"  ), b"0" ) );
    assert!( charClassMatch( class!( "a-cdefgh-k" ), b"e" ) );
    assert!( charClassMatch( class!( "---"        ), b"-" ) );
    assert!( charClassMatch( class!( "a-a"        ), b"a" ) );
  }

  #[test]
  fn CharClass_Match_NonUnicode() {
    assert!( charClassMatch( &CharClass::new( &[255] ), &[255] ) );
  }


  #[test]
  fn CharClass_NoMatch() {
    assert!( !charClassMatch( class!( "a"   ), b"b" ) );
    assert!( !charClassMatch( class!( "-"   ), b"a" ) );
    assert!( !charClassMatch( class!( "z-a" ), b"a" ) );
    assert!( !charClassMatch( class!( "z-a" ), b"b" ) );
    assert!( !charClassMatch( class!( "a-z" ), b"0" ) );
    assert!( !charClassMatch( class!( "a-z" ), b"A" ) );
  }


  // TODO: tests for escaped chars in class
}
