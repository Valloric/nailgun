use super::{Expression, ParseState, ParseResult};

pub struct AndExpression<'a> {
  expr: &'a Expression
}


impl<'a> AndExpression<'a> {
  pub fn new( expr: &'a Expression ) -> AndExpression<'a> {
    AndExpression { expr: expr }
  }
}


impl<'a> Expression for AndExpression<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    match self.expr.apply( parse_state ) {
      Some( _ ) => Some( ParseResult::fromParseState( *parse_state ) ),
      _ => None
    }
  }
}


#[cfg(test)]
mod tests {
  use base::{ParseResult, Expression};
  use base::literal::LiteralExpression;
  use base::char_class::CharClassExpression;
  use base::test_utils::ToParseState;
  use super::AndExpression;

  #[test]
  fn AndExpression_Match_WithLiteral() {
    byte_var!(input = "foo");
    byte_var!(literal = "foo");
    let orig_state = ToParseState( input );
    match AndExpression::new( &LiteralExpression::new( literal ) ).apply(
        &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert!( nodes.is_empty() );
        assert_eq!( parse_state, orig_state );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn AndExpression_Match_WithCharClass() {
    byte_var!(input = "c");
    let orig_state = ToParseState( input );
    match AndExpression::new(
      &CharClassExpression::new( bytes!( "a-z" ) ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert!( nodes.is_empty() );
        assert_eq!( parse_state, orig_state );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn AndExpression_NoMatch() {
    assert!( AndExpression::new(
        &CharClassExpression::new( bytes!( "a-z" ) ) ).apply(
        &ToParseState( bytes!( "0" ) ) ).is_none() )

    byte_var!(literal = "x");
    assert!( AndExpression::new( &LiteralExpression::new( literal ) ).apply(
        &ToParseState( bytes!( "y" ) ) ).is_none() )
  }
}

