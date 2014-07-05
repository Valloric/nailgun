use super::{Expression, ParseState, ParseResult};

macro_rules! not( ( $ex:expr ) => ( {
    use base;
    base::NotEx::new(& $ex) } ); )

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

  #[test]
  fn NotEx_Match_WithLiteral() {
    let orig_state = input_state!( "zoo" );
    match not!( lit!( "foo" ) ).apply( &orig_state ) {
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
    let orig_state = input_state!( "0" );
    match not!( class!( "a-z" ) ).apply( &orig_state ) {
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
    assert!( not!( class!( "a-z" ) ).apply( &input_state!( "b" ) ).is_none() )
    assert!( not!( lit!( "x" ) ).apply( &input_state!( "x" ) ).is_none() )
  }
}
