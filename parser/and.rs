use super::{Expression, Node, ParseState, ParseResult};

static AND_EXPRESSION : &'static str = "AndExpression";

pub struct AndExpression {
  expr: ~Expression
}


impl AndExpression {
  pub fn new( expr: ~Expression ) -> AndExpression {
    AndExpression { expr: expr }
  }
}


impl Expression for AndExpression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    match self.expr.apply( parse_state ) {
      Some( _ ) => Some(
        ParseResult::oneNode( Node::predicate( AND_EXPRESSION ),
                              *parse_state ) ),
      _ => None
    }
  }
}


#[cfg(test)]
mod tests {
  use parser::{Node, ParseResult, Expression};
  use parser::literal::LiteralExpression;
  use parser::char_class::CharClassExpression;
  use parser::test_utils::ToParseState;
  use super::{AND_EXPRESSION, AndExpression};

  #[test]
  fn AndExpression_Match_WithLiteral() {
    match AndExpression::new( ~LiteralExpression::new( "foo" ) ).apply(
        &ToParseState( "foo" ) ) {
      Some( ParseResult{ nodes: ref nodes,
                        parse_state: mut parse_state } ) => {
        assert_eq!( *nodes.get( 0 ).unwrap(), Node::predicate( AND_EXPRESSION ) );
        assert_eq!( parse_state.next(), Some( ( 0, 'f' ) ) );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn AndExpression_Match_WithCharClass() {
    match AndExpression::new( ~CharClassExpression::new( "a-z" ) ).apply(
        &ToParseState( "c" ) ) {
      Some( ParseResult{ nodes: ref nodes,
                        parse_state: mut parse_state } ) => {
        assert_eq!( *nodes.get( 0 ).unwrap(), Node::predicate( AND_EXPRESSION ) );
        assert_eq!( parse_state.next(), Some( ( 0, 'c' ) ) );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn AndExpression_NoMatch() {
    assert!( AndExpression::new( ~CharClassExpression::new( "a-z" ) ).apply(
        &ToParseState( "0" ) ).is_none() )
    assert!( AndExpression::new( ~LiteralExpression::new( "x" ) ).apply(
        &ToParseState( "y" ) ).is_none() )
  }
}

