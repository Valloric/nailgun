use super::{Expression, ParseState, ParseResult};

macro_rules! and( ( $ex:expr ) => ( And::new( $ex ) ); )

pub struct And<'a> {
  expr: &'a Expression
}


impl<'a> And<'a> {
  pub fn new( expr: &'a Expression ) -> And<'a> {
    And { expr: expr }
  }
}


impl<'a> Expression for And<'a> {
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
  use base::literal::Literal;
  use base::char_class::CharClass;
  use base::test_utils::ToParseState;
  use super::And;

  #[test]
  fn And_Match_WithLiteral() {
    byte_var!(input = "foo");
    let orig_state = ToParseState( input );
    match and!( &lit!( "foo" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert!( nodes.is_empty() );
        assert_eq!( parse_state, orig_state );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn And_Match_WithCharClass() {
    byte_var!(input = "c");
    let orig_state = ToParseState( input );
    match and!( &CharClass::new( bytes!( "a-z" ) ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert!( nodes.is_empty() );
        assert_eq!( parse_state, orig_state );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn And_NoMatch() {
    assert!( and!( &CharClass::new( bytes!( "a-z" ) ) ).apply(
        &ToParseState( bytes!( "0" ) ) ).is_none() )

    assert!( and!( &lit!( "x" ) ).apply(
        &ToParseState( bytes!( "y" ) ) ).is_none() )
  }
}

