use super::{Expression, ParseState, ParseResult};

pub struct StarExpression<'a> {
  expr: &'a Expression
}


impl<'a> StarExpression<'a> {
  pub fn new<'a>( expr: &'a Expression ) -> StarExpression<'a> {
    StarExpression { expr: expr }
  }
}


impl<'a> Expression for StarExpression<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    let mut final_result = ParseResult::fromParseState( *parse_state );
    loop {
      match self.expr.apply( &final_result.parse_state ) {
        Some( result ) => {
          final_result.parse_state = result.parse_state;
          final_result.nodes.push_all_move( result.nodes );
        }
        _ => break
      }
    }
    Some( final_result )
  }
}


#[cfg(test)]
mod tests {
  use parser::{Node, ParseResult, Expression, Data};
  use parser::literal::{LiteralExpression, LITERAL_EXPRESSION};
  use parser::test_utils::ToParseState;
  use super::{StarExpression};

  #[test]
  fn StarExpression_Match() {
    byte_var!(input = "aaa");
    byte_var!(literal = "a");
    let orig_state = ToParseState( input );
    match StarExpression::new( &LiteralExpression::new( literal ) ).apply(
        &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 0,
                           end: 1,
                           contents: Data( literal ) } );
        assert_eq!( *nodes.get( 1 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 1,
                           end: 2,
                           contents: Data( literal ) } );
        assert_eq!( *nodes.get( 2 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 2,
                           end: 3,
                           contents: Data( literal ) } );
        assert_eq!( parse_state, orig_state.advanceTo( 3 ) );
      }
      _ => fail!( "No match." )
    }
  }

  fn StarExpression_Match_JustOne() {
    byte_var!(input = "abb");
    byte_var!(literal = "a");
    let orig_state = ToParseState( input );
    match StarExpression::new( &LiteralExpression::new( literal ) ).apply(
        &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 0,
                           end: 1,
                           contents: Data( literal ) } );
        assert_eq!( parse_state, orig_state.advanceTo( 1 ) );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn StarExpression_Match_Empty() {
    byte_var!(input = "y");
    byte_var!(literal = "x");
    let orig_state = ToParseState( input );
    match StarExpression::new( &LiteralExpression::new( literal ) ).apply(
        &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert!( nodes.is_empty() );
        assert_eq!( parse_state, orig_state );
      }
      _ => fail!( "No match." )
    }
  }
}
