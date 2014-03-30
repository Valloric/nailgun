use base::unicode::{bytesFollowing, readCodepoint};
use base::unescape::unescape;
use super::{Expression, ParseState, ParseResult};

macro_rules! class( ( $ex:expr ) => ( {
      use base;
      byte_var!( input = $ex )
      base::CharClass::new( input )
    } ) )

static CHAR_CLASS_EXPRESSION : &'static str = "CharClass";

fn toU32Vector( input: &[u8] ) -> Vec<u32> {
  let mut i = 0;
  let mut out_vec : Vec<u32> = vec!();
  loop {
    match input.get( i ) {
      Some( byte ) => match bytesFollowing( *byte ) {
        Some( num_following ) => {
          if num_following > 0 {
            match readCodepoint( input.slice_from( i ) ) {
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
    fn rangeAtIndex( index: uint, chars: &[u32] ) -> Option<( u32, u32 )> {
      match ( chars.get( index ),
              chars.get( index + 1 ),
              chars.get( index + 2 ) ) {
        ( Some( char1 ), Some( char2 ), Some( char3 ) )
            if *char2 == '-' as u32 => Some( ( *char1, *char3 ) ),
        _ => None
      }
    }

    // TODO: instead of unescaping the input here, the passed-in value should be
    // unescaped already by the peg code generator.
    let chars = toU32Vector( unescape( contents ).as_slice() );
    let mut char_class = CharClass { single_chars: Vec::new(),
                                               ranges: Vec::new() };
    let mut index = 0;
    loop {
      match rangeAtIndex( index, chars.as_slice() ) {
        Some( range ) => {
          char_class.ranges.push( range );
          index += 3;
        }
        _ => {
          if index >= chars.len() {
            break
          }
          char_class.single_chars.push( *chars.get( index ) );
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
        parse_state.nameAndOffsetToResult(
          CHAR_CLASS_EXPRESSION, parse_state.offset + num_following + 1 )
      }
      _ => None
    }
  }


  fn applyToBytes<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    match parse_state.input.get( 0 ) {
      Some( byte ) if self.matches( *byte as u32 ) => {
        parse_state.nameAndOffsetToResult(
          CHAR_CLASS_EXPRESSION, parse_state.offset + 1 )
      }
      _ => None
    }
  }
}


impl Expression for CharClass {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    match self.applyToUtf8( parse_state ) {
      Some( x ) => Some( x ),
      _ => self.applyToBytes( parse_state )
    }
  }
}


#[cfg(test)]
mod tests {
  use base::{Node, Data, ParseResult, Expression, ParseState};
  use base::test_utils::ToParseState;
  use base::unicode::bytesFollowing;
  use super::{CHAR_CLASS_EXPRESSION, CharClass};

  fn charClassMatch( char_class: CharClass, input: &[u8] ) -> bool {
    fn bytesRead( input: &[u8] ) -> uint {
      match bytesFollowing( input[ 0 ] ) {
        Some( num_following ) => num_following + 1,
        _ => 1
      }
    }

    match char_class.apply( &ToParseState( input ) ) {
      Some( ParseResult { nodes: nodes,
                          parse_state: parse_state } ) => {
        let bytes_read = bytesRead( input );
        assert_eq!( *nodes.get( 0 ),
                    Node { name: CHAR_CLASS_EXPRESSION,
                           start: 0,
                           end: bytes_read,
                           contents: Data( input ) } );
        assert_eq!( parse_state, ParseState{ input: &[], offset: bytes_read } );
        true
      }
      _ => false
    }
  }


  #[test]
  fn CharClass_Match() {
    assert!( charClassMatch( class!( "a"          ), bytes!( "a" ) ) );
    assert!( charClassMatch( class!( "abcdef"     ), bytes!( "e" ) ) );
    assert!( charClassMatch( class!( "a-z"        ), bytes!( "a" ) ) );
    assert!( charClassMatch( class!( "a-z"        ), bytes!( "c" ) ) );
    assert!( charClassMatch( class!( "a-z"        ), bytes!( "z" ) ) );
    assert!( charClassMatch( class!( "0-9"        ), bytes!( "2" ) ) );
    assert!( charClassMatch( class!( "α-ω"        ), bytes!( "η" ) ) );
    assert!( charClassMatch( class!( "-"          ), bytes!( "-" ) ) );
    assert!( charClassMatch( class!( "a-"         ), bytes!( "-" ) ) );
    assert!( charClassMatch( class!( "-a"         ), bytes!( "-" ) ) );
    assert!( charClassMatch( class!( "a-zA-Z-"    ), bytes!( "-" ) ) );
    assert!( charClassMatch( class!( "aa-zA-Z-a"  ), bytes!( "-" ) ) );
    assert!( charClassMatch( class!( "a-zA-Z-"    ), bytes!( "z" ) ) );
    assert!( charClassMatch( class!( "aa-zA-Z-0"  ), bytes!( "0" ) ) );
    assert!( charClassMatch( class!( "a-cdefgh-k" ), bytes!( "e" ) ) );
    assert!( charClassMatch( class!( "---"        ), bytes!( "-" ) ) );
    assert!( charClassMatch( class!( "a-a"        ), bytes!( "a" ) ) );
  }

  // TODO: Use these tests for the char-class unescape code that will be written
  // in the code generator.
  // #[test]
  // fn CharClass_Match_ClassEscapedChars() {
  //   assert!( charClassMatch( class!( r"\]" ), bytes!( "]" ) ) );
  //   assert!( charClassMatch( class!( r"\\" ), bytes!( r"\" ) ) );
  //   assert!( !charClassMatch( class!( r"\]" ), bytes!( r"\" ) ) );
  // }

  #[test]
  fn CharClass_Match_NonUnicode() {
    assert!( charClassMatch( CharClass::new( [255] ), [255] ) );
  }


  #[test]
  fn CharClass_NoMatch() {
    assert!( !charClassMatch( class!( "a"   ), bytes!( "b" ) ) );
    assert!( !charClassMatch( class!( "-"   ), bytes!( "a" ) ) );
    assert!( !charClassMatch( class!( "z-a" ), bytes!( "a" ) ) );
    assert!( !charClassMatch( class!( "z-a" ), bytes!( "b" ) ) );
    assert!( !charClassMatch( class!( "a-z" ), bytes!( "0" ) ) );
    assert!( !charClassMatch( class!( "a-z" ), bytes!( "A" ) ) );
  }


  // TODO: tests for escaped chars in class
}