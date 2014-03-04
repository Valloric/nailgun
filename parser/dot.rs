use std::str::from_char;
use super::{Expression, Data, Node, ParseState, ParseResult};

static DOT_EXPRESSION : &'static str = "DotExpression";

pub struct DotExpression;
impl Expression for DotExpression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) -> Option< ParseResult<'a> > {
    let mut new_parse_state = parse_state.clone();
    match new_parse_state.next() {
      Some( ( index, character ) ) => Some(
        ParseResult::oneNode( Node { name: DOT_EXPRESSION,
                                     start: index,
                                     end: index + 1,
                                     contents: Data( from_char( character ) ) },
                              new_parse_state ) ),
      _ => None
    }
  }
}

#[cfg(test)]
mod tests {
  use parser::test_utils::ToParseState;
  use super::{DotExpression, DOT_EXPRESSION};
  use parser::{Node, Data, ParseResult, Expression};

  #[test]
  fn DotExpression_Match_InputOneChar() {
    match DotExpression.apply( &ToParseState( "x" ) ) {
      Some( ParseResult{ nodes: ref nodes,
                        parse_state: mut parse_state } ) => {
        assert_eq!( *nodes.get( 0 ).unwrap(),
                    Node { name: DOT_EXPRESSION,
                          start: 0,
                          end: 1,
                          contents: Data( ~"x" ) } );
        assert_eq!( parse_state.next(), None );
      }
      _ => fail!( "No match!" )
    };
  }


  #[test]
  fn DotExpression_Match_InputOneWideChar() {
    match DotExpression.apply( &ToParseState( "葉" ) ) {
      Some( ParseResult{ nodes: ref nodes,
                        parse_state: mut parse_state } ) => {
        assert_eq!( *nodes.get( 0 ).unwrap(),
                    Node { name: DOT_EXPRESSION,
                          start: 0,
                          end: 1,
                          contents: Data( ~"葉" ) } );
        assert_eq!( parse_state.next(), None );
      }
      _ => fail!( "No match!" )
    };
  }


  #[test]
  fn DotExpression_Match_InputSeveralChars() {
    match DotExpression.apply( &ToParseState( "xb" ) ) {
      Some( ParseResult{ nodes: ref nodes,
                        parse_state: mut parse_state } ) => {
        assert_eq!( *nodes.get( 0 ).unwrap(),
                    Node { name: DOT_EXPRESSION,
                          start: 0,
                          end: 1,
                          contents: Data( ~"x" ) } );
        assert_eq!( parse_state.next(), Some( ( 1, 'b' ) ) );
      }
      _ => fail!( "No match!" )
    };
  }


  #[test]
  fn DotExpression_NoMatch() {
    assert!( DotExpression.apply( &ToParseState( "" ) ).is_none() )
  }
}
