use super::{Expression, ParseState, ParseResult};
use base::unicode::{bytesFollowing, readCodepoint};

static DOT_EXPRESSION : &'static str = "Dot";

pub struct Dot;
impl Expression for Dot {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) -> Option< ParseResult<'a> > {
    match readCodepoint( parse_state.input ) {
      Some( _ ) => {
        let num_following = bytesFollowing( parse_state.input[ 0 ] ).unwrap();
        return parse_state.nameAndOffsetToResult(
          DOT_EXPRESSION, parse_state.offset + num_following + 1 )
      }
      _ => ()
    }

    match parse_state.input.get( 0 ) {
      Some( _ ) => parse_state.nameAndOffsetToResult( DOT_EXPRESSION,
                                                      parse_state.offset + 1 ),
      _ => None
    }
  }
}


#[cfg(test)]
mod tests {
  use base::test_utils::ToParseState;
  use super::{Dot, DOT_EXPRESSION};
  use base::{Node, Data, ParseResult, ParseState, Expression};

  #[test]
  fn Dot_Match_InputOneChar() {
    byte_var!(input = "x");
    match Dot.apply( &ToParseState( input ) ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node { name: DOT_EXPRESSION,
                           start: 0,
                           end: 1,
                           contents: Data( input ) } );
        assert_eq!( parse_state, ParseState{ input: &[], offset: 1 } );
      }
      _ => fail!( "No match!" )
    };
  }


  #[test]
  fn Dot_Match_InputOneWideChar() {
    byte_var!(input = "葉");
    match Dot.apply( &ToParseState( input ) ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node { name: DOT_EXPRESSION,
                           start: 0,
                           end: 3,
                           contents: Data( input ) } );
        assert_eq!( parse_state, ParseState{ input: &[], offset: 3 } );
      }
      _ => fail!( "No match!" )
    };
  }


  #[test]
  fn Dot_Match_InputSeveralChars() {
    byte_var!(input = "xb");
    byte_var!(consumed = "x");
    match Dot.apply( &ToParseState( input ) ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert!( *nodes.get( 0 ) ==
                 Node { name: DOT_EXPRESSION,
                        start: 0,
                        end: 1,
                        contents: Data( consumed ) } );
        assert_eq!( parse_state, ParseState{ input: bytes!( "b" ),
                                             offset: 1 } );
      }
      _ => fail!( "No match!" )
    };
  }


  #[test]
  fn Dot_NoMatch() {
    assert!( Dot.apply( &ToParseState( bytes!( "" ) ) ).is_none() )
  }
}
