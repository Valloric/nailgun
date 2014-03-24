use super::{Expression, ParseState, ParseResult};

pub struct OptionExpression<'a> {
  expr: &'a Expression
}


impl<'a> OptionExpression<'a> {
  pub fn new( expr: &'a Expression ) -> OptionExpression<'a> {
    OptionExpression { expr: expr }
  }
}


impl<'a> Expression for OptionExpression<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    match self.expr.apply( parse_state ) {
      result @ Some( _ ) => result,
      _ => Some( ParseResult::fromParseState( *parse_state ) )
    }
  }
}


#[cfg(test)]
mod tests {
  use parser::{Node, ParseResult, Expression, Data};
  use parser::literal::{LiteralExpression, LITERAL_EXPRESSION};
  use parser::test_utils::ToParseState;
  use super::{OptionExpression};

  #[test]
  fn OptionExpression_Match_WithLiteral() {
    byte_var!(input = "foo");
    byte_var!(literal = "foo");
    let orig_state = ToParseState( input );
    match OptionExpression::new( &LiteralExpression::new( literal ) ).apply(
        &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 0,
                           end: 3,
                           contents: Data( literal ) } );
        assert_eq!( parse_state, orig_state.advanceTo( 3 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn OptionExpression_Match_Empty() {
    byte_var!(input = "y");
    byte_var!(literal = "x");
    let orig_state = ToParseState( input );
    match OptionExpression::new( &LiteralExpression::new( literal ) ).apply(
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


