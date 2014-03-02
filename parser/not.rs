use super::{Expression, Node, ParseState, ParseResult};

static NOT_EXPRESSION : &'static str = "NotExpression";

pub struct NotExpression {
  expr: ~Expression
}


impl NotExpression {
  pub fn new( expr: ~Expression ) -> NotExpression {
    NotExpression { expr: expr }
  }
}


impl Expression for NotExpression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    match self.expr.apply( parse_state ) {
      Some( _ ) => None,
      _ => Some(
        ParseResult::oneNode( Node::predicate( NOT_EXPRESSION ),
                              *parse_state ) )
    }
  }
}


#[cfg(test)]
mod tests {
  use parser::{Node, ParseResult, Expression};
  use parser::literal::LiteralExpression;
  use parser::char_class::CharClassExpression;
  use parser::test_utils::ToParseState;
  use super::{NOT_EXPRESSION, NotExpression};

  #[test]
  fn NotExpression_Match_WithLiteral() {
    match NotExpression::new( ~LiteralExpression::new( "foo" ) ).apply(
        &ToParseState( "zoo" ) ) {
      Some( ParseResult{ nodes: ref nodes,
                        parse_state: mut parse_state } ) => {
        assert_eq!( *nodes.get( 0 ).unwrap(), Node::predicate( NOT_EXPRESSION ) );
        assert_eq!( parse_state.next(), Some( ( 0, 'z' ) ) );
      }
      _ => fail!( "No match." )
    }

    assert!( NotExpression::new( ~CharClassExpression::new( "a-z" ) )
            .apply( &ToParseState( "0" ) ).is_some() )
  }


  #[test]
  fn NotExpression_Match_WithCharClass() {
    match NotExpression::new( ~CharClassExpression::new( "a-z" ) ).apply(
        &ToParseState( "0" ) ) {
      Some( ParseResult{ nodes: ref nodes,
                        parse_state: mut parse_state } ) => {
        assert_eq!( *nodes.get( 0 ).unwrap(), Node::predicate( NOT_EXPRESSION ) );
        assert_eq!( parse_state.next(), Some( ( 0, '0' ) ) );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn NotExpression_NoMatch() {
    assert!( NotExpression::new( ~CharClassExpression::new( "a-z" ) ).apply(
        &ToParseState( "b" ) ).is_none() )
    assert!( NotExpression::new( ~LiteralExpression::new( "x" ) ).apply(
        &ToParseState( "x" ) ).is_none() )
  }
}
