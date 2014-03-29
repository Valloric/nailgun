use super::{Expression, ParseState, ParseResult};

pub struct SequenceExpression<'a> {
  exprs: &'a [&'a Expression]
}


impl<'a> SequenceExpression<'a> {
  pub fn new<'a>( exprs: &'a [&Expression] ) -> SequenceExpression<'a> {
    SequenceExpression { exprs: exprs }
  }
}


impl<'a> Expression for SequenceExpression<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    let mut final_result = ParseResult::fromParseState( *parse_state );
    for expr in self.exprs.iter() {
      match expr.apply( &final_result.parse_state ) {
        Some( result ) => {
          final_result.parse_state = result.parse_state;
          final_result.nodes.push_all_move( result.nodes );
        }
        _ => return None
      }
    }
    Some( final_result )
  }
}


#[cfg(test)]
mod tests {
  use base::{Node, ParseResult, Expression, Data};
  use base::literal::{LiteralExpression, LITERAL_EXPRESSION};
  use base::test_utils::ToParseState;
  use super::{SequenceExpression};

  #[test]
  fn SequenceExpression_Match() {
    byte_var!(input = "ab");
    byte_var!(literal1 = "a");
    byte_var!(literal2 = "b");
    let orig_state = ToParseState( input );
    match SequenceExpression::new(
      &[&LiteralExpression::new( literal1 ) as &Expression,
        &LiteralExpression::new( literal2 ) as &Expression] ).apply(
          &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 0,
                           end: 1,
                           contents: Data( literal1 ) } );
        assert_eq!( *nodes.get( 1 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 1,
                           end: 2,
                           contents: Data( literal2 ) } );
        assert_eq!( parse_state, orig_state.advanceTo( 2 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn SequenceExpression_NoMatch() {
    byte_var!(input = "aa");
    byte_var!(literal1 = "a");
    byte_var!(literal2 = "b");
    let orig_state = ToParseState( input );

    assert!( SequenceExpression::new(
      &[&LiteralExpression::new( literal1 ) as &Expression,
        &LiteralExpression::new( literal2 ) as &Expression] ).apply(
          &orig_state ).is_none() )
  }
}
