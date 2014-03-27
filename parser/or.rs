use super::{Expression, ParseState, ParseResult};

pub struct OrExpression<'a> {
  exprs: &'a [&'a Expression]
}


impl<'a> OrExpression<'a> {
  pub fn new<'a>( exprs: &'a [&Expression] ) -> OrExpression<'a> {
    OrExpression { exprs: exprs }
  }
}


impl<'a> Expression for OrExpression<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    for expr in self.exprs.iter() {
      match expr.apply( parse_state ) {
        result @ Some( _ ) => return result,
        _ => ()
      }
    }
    None
  }
}


#[cfg(test)]
mod tests {
  use parser::{Node, ParseResult, Expression, Data};
  use parser::literal::{LiteralExpression, LITERAL_EXPRESSION};
  use parser::test_utils::ToParseState;
  use super::{OrExpression};

  #[test]
  fn OrExpression_Match_FirstExpr() {
    byte_var!(input = "a");
    byte_var!(literal1 = "a");
    byte_var!(literal2 = "b");
    let orig_state = ToParseState( input );
    match OrExpression::new(
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
        assert_eq!( parse_state, orig_state.advanceTo( 1 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn OrExpression_Match_SecondExpr() {
    byte_var!(input = "a");
    byte_var!(literal1 = "b");
    byte_var!(literal2 = "a");
    let orig_state = ToParseState( input );
    match OrExpression::new(
      &[&LiteralExpression::new( literal1 ) as &Expression,
        &LiteralExpression::new( literal2 ) as &Expression] ).apply(
          &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 0,
                           end: 1,
                           contents: Data( literal2 ) } );
        assert_eq!( parse_state, orig_state.advanceTo( 1 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn OrExpression_Match_FirstExprIfBoth() {
    byte_var!(input = "a");
    byte_var!(literal1 = "a");
    byte_var!(literal2 = "a");
    let orig_state = ToParseState( input );
    match OrExpression::new(
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
        assert_eq!( parse_state, orig_state.advanceTo( 1 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn OrExpression_NoMatch() {
    byte_var!(input = "a");
    byte_var!(literal1 = "b");
    byte_var!(literal2 = "c");
    let orig_state = ToParseState( input );

    assert!( OrExpression::new(
      &[&LiteralExpression::new( literal1 ) as &Expression,
        &LiteralExpression::new( literal2 ) as &Expression] ).apply(
          &orig_state ).is_none() )
  }
}

