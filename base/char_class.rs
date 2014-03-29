use base::unicode::{bytesFollowing, readCodepoint};
use base::unescape::unescape;
use super::{Expression, ParseState, ParseResult};

static CHAR_CLASS_EXPRESSION : &'static str = "CharClassExpression";

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


pub struct CharClassExpression {
  // All the single chars in the char class.
  // May be unicode codepoints or binary octets stored as codepoints.
  single_chars: Vec<u32>,

  // Sequence of [from, to] (inclusive bounds) char ranges.
  // May be unicode codepoints or binary octets stored as codepoints.
  ranges: Vec<( u32, u32 )>
}


impl CharClassExpression {
  // Takes the inner content of square brackets, so for [a-z], send "a-z".
  pub fn new( contents: &[u8] ) -> CharClassExpression {
    fn rangeAtIndex( index: uint, chars: &[u32] ) -> Option<( u32, u32 )> {
      match ( chars.get( index ),
              chars.get( index + 1 ),
              chars.get( index + 2 ) ) {
        ( Some( char1 ), Some( char2 ), Some( char3 ) )
            if *char2 == '-' as u32 => Some( ( *char1, *char3 ) ),
        _ => None
      }
    }

    let chars = toU32Vector( unescape( contents ).as_slice() );
    let mut char_class = CharClassExpression { single_chars: Vec::new(),
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


impl Expression for CharClassExpression {
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
  use super::{CHAR_CLASS_EXPRESSION, CharClassExpression};

  fn charClassMatch( char_class: CharClassExpression, input: &[u8] ) -> bool {
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
  fn CharClassExpression_Match() {
    assert!( charClassMatch( CharClassExpression::new( bytes!( "a" ) ),
                             bytes!( "a" ) ) );
    assert!( charClassMatch( CharClassExpression::new( bytes!( "abcdef" ) ),
                             bytes!( "e" ) ) );
    assert!( charClassMatch( CharClassExpression::new( bytes!( "a-z" ) ),
                             bytes!( "a" ) ) );
    assert!( charClassMatch( CharClassExpression::new( bytes!( "a-z" ) ),
                             bytes!( "c" ) ) );
    assert!( charClassMatch( CharClassExpression::new( bytes!( "a-z" ) ),
                             bytes!( "z" ) ) );
    assert!( charClassMatch( CharClassExpression::new( bytes!( "0-9" ) ),
                             bytes!( "2" ) ) );
    assert!( charClassMatch( CharClassExpression::new( bytes!( "α-ω" ) ),
                             bytes!( "η" ) ) );
    assert!( charClassMatch( CharClassExpression::new( bytes!( "-" ) ),
                             bytes!( "-" ) ) );
    assert!( charClassMatch( CharClassExpression::new( bytes!( "a-" ) ),
                             bytes!( "-" ) ) );
    assert!( charClassMatch( CharClassExpression::new( bytes!( "-a" ) ),
                             bytes!( "-" ) ) );
    assert!( charClassMatch( CharClassExpression::new( bytes!( "a-zA-Z-" ) ),
                             bytes!( "-" ) ) );
    assert!( charClassMatch( CharClassExpression::new( bytes!( "aa-zA-Z-a" ) ),
                             bytes!( "-" ) ) );
    assert!( charClassMatch( CharClassExpression::new( bytes!( "a-zA-Z-" ) ),
                             bytes!( "z" ) ) );
    assert!( charClassMatch( CharClassExpression::new( bytes!( "aa-zA-Z-0" ) ),
                             bytes!( "0" ) ) );
    assert!( charClassMatch( CharClassExpression::new( bytes!( "a-cdefgh-k" ) ),
                             bytes!( "e" ) ) );
    assert!( charClassMatch( CharClassExpression::new( bytes!( "---" ) ),
                             bytes!( "-" ) ) );
    assert!( charClassMatch( CharClassExpression::new( bytes!( "a-a" ) ),
                             bytes!( "a" ) ) );
  }


  #[test]
  fn CharClassExpression_Match_NonUnicode() {
    assert!( charClassMatch( CharClassExpression::new( [255] ), [255] ) );
  }


  #[test]
  fn CharClassExpression_NoMatch() {
    assert!( !charClassMatch( CharClassExpression::new( bytes!( "a" ) ),
                             bytes!( "b" ) ) );
    assert!( !charClassMatch( CharClassExpression::new( bytes!( "-" ) ),
                             bytes!( "a" ) ) );
    assert!( !charClassMatch( CharClassExpression::new( bytes!( "z-a" ) ),
                             bytes!( "a" ) ) );
    assert!( !charClassMatch( CharClassExpression::new( bytes!( "z-a" ) ),
                             bytes!( "b" ) ) );
    assert!( !charClassMatch( CharClassExpression::new( bytes!( "a-z" ) ),
                             bytes!( "0" ) ) );
    assert!( !charClassMatch( CharClassExpression::new( bytes!( "a-z" ) ),
                             bytes!( "A" ) ) );
  }


  // TODO: tests for escaped chars in class
}
