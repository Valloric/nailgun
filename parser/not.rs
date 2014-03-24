use super::{Expression, ParseState, ParseResult};

pub struct NotExpression<'a> {
  expr: &'a Expression
}


impl<'a> NotExpression<'a> {
  pub fn new<'a>( expr: &'a Expression ) -> NotExpression<'a> {
    NotExpression { expr: expr }
  }
}


impl<'a> Expression for NotExpression<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    match self.expr.apply( parse_state ) {
      Some( _ ) => None,
      _ => Some( ParseResult::fromParseState( *parse_state ) )
    }
  }
}


#[cfg(test)]
mod tests {
  use parser::{ParseResult, Expression};
  use parser::literal::LiteralExpression;
  use parser::char_class::CharClassExpression;
  use parser::test_utils::ToParseState;
  use super::NotExpression;

  #[test]
  fn NotExpression_Match_WithLiteral() {
    byte_var!(input = "zoo");
    byte_var!(literal = "foo");
    let orig_state = ToParseState( input );
    match NotExpression::new( &LiteralExpression::new( literal ) ).apply(
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
  fn NotExpression_Match_WithCharClass() {
    // TODO: macro for creating parse state? (auto-create static var)
    // similar code in and.rs
    byte_var!(input = "0");
    let orig_state = ToParseState( input );
    match NotExpression::new(
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
  fn NotExpression_NoMatch() {
    assert!( NotExpression::new(
        &CharClassExpression::new( bytes!( "a-z" ) ) ).apply(
        &ToParseState( bytes!( "b" ) ) ).is_none() )

    byte_var!(literal = "x");
    assert!( NotExpression::new( &LiteralExpression::new( literal ) ).apply(
        &ToParseState( bytes!( "x" ) ) ).is_none() )
  }
}
