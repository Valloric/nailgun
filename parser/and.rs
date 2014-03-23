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
    byte_var!(input = "foo");
    byte_var!(literal = "foo");
    let orig_state = ToParseState( input );
    match AndExpression::new( ~LiteralExpression::new( literal ) ).apply(
        &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node::predicate( AND_EXPRESSION ) );
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
      ~CharClassExpression::new( bytes!( "a-z" ) ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node::predicate( AND_EXPRESSION ) );
        assert_eq!( parse_state, orig_state );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn AndExpression_NoMatch() {
    assert!( AndExpression::new(
        ~CharClassExpression::new( bytes!( "a-z" ) ) ).apply(
        &ToParseState( bytes!( "0" ) ) ).is_none() )

    byte_var!(literal = "x");
    assert!( AndExpression::new( ~LiteralExpression::new( literal ) ).apply(
        &ToParseState( bytes!( "y" ) ) ).is_none() )
  }
}

