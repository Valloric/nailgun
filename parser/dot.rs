use super::{Expression, ParseState, ParseResult};
use parser::unicode::{bytesFollowing, readCodepoint};

static DOT_EXPRESSION : &'static str = "DotExpression";

pub struct DotExpression;
impl Expression for DotExpression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) -> Option< ParseResult<'a> > {
    match readCodepoint( parse_state.input ) {
      Some( ch ) => {
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
  use parser::test_utils::ToParseState;
  use super::{DotExpression, DOT_EXPRESSION};
  use parser::{Node, Data, ParseResult, ParseState, Expression};

  #[test]
  fn DotExpression_Match_InputOneChar() {
    static input: &'static [u8] = bytes!( "x" );
    match DotExpression.apply( &ToParseState( input ) ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ).unwrap(),
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
  fn DotExpression_Match_InputOneWideChar() {
    static input: &'static [u8] = bytes!( "葉" );
    match DotExpression.apply( &ToParseState( input ) ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ).unwrap(),
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
  fn DotExpression_Match_InputSeveralChars() {
    static input: &'static [u8] = bytes!( "xb" );
    match DotExpression.apply( &ToParseState( input ) ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert!( *nodes.get( 0 ).unwrap() ==
                 Node { name: DOT_EXPRESSION,
                        start: 0,
                        end: 1,
                        contents: Data( bytes!( "x" ) ) } );
        assert_eq!( parse_state, ParseState{ input: bytes!( "b" ),
                                             offset: 1 } );
      }
      _ => fail!( "No match!" )
    };
  }


  #[test]
  fn DotExpression_NoMatch() {
    assert!( DotExpression.apply( &ToParseState( bytes!( "" ) ) ).is_none() )
  }
}
