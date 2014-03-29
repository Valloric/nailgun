use super::{Expression, ParseState, ParseResult};

macro_rules! not( ( $ex:expr ) => ( NotEx::new($ex) ); )

pub struct NotEx<'a> {
  expr: &'a Expression
}


impl<'a> NotEx<'a> {
  pub fn new<'a>( expr: &'a Expression ) -> NotEx<'a> {
    NotEx { expr: expr }
  }
}


impl<'a> Expression for NotEx<'a> {
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
  use base::{ParseResult, Expression};
  use base::literal::Literal;
  use base::char_class::CharClass;
  use base::test_utils::ToParseState;
  use super::NotEx;

  #[test]
  fn NotEx_Match_WithLiteral() {
    byte_var!(input = "zoo");
    let orig_state = ToParseState( input );
    match not!( &lit!( "foo" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert!( nodes.is_empty() );
        assert_eq!( parse_state, orig_state );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn NotEx_Match_WithCharClass() {
    // TODO: macro for creating parse state? (auto-create static var)
    // similar code in and.rs
    byte_var!(input = "0");
    let orig_state = ToParseState( input );
    match not!( &CharClass::new( bytes!( "a-z" ) ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                        parse_state: parse_state } ) => {
        assert!( nodes.is_empty() );
        assert_eq!( parse_state, orig_state );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn NotEx_NoMatch() {
    assert!( not!( &CharClass::new( bytes!( "a-z" ) ) ).apply(
        &ToParseState( bytes!( "b" ) ) ).is_none() )

    assert!( not!( &lit!( "x" ) ).apply(
        &ToParseState( bytes!( "x" ) ) ).is_none() )
  }
}
