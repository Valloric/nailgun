// TODO: Remove this in the future.
#![allow(dead_code)]

use super::{Expression, ParseState, ParseResult};

macro_rules! and( ( $ex:expr ) => ( {
    use base;
    base::And::new( & $ex ) } ); )

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

  #[test]
  fn And_Match_WithLiteral() {
    let orig_state = input_state!( "foo" );
    match and!( lit!( "foo" ) ).apply( &orig_state ) {
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
    let orig_state = input_state!( "c" );
    match and!( class!( "a-z" ) ).apply( &orig_state ) {
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
    assert!( and!( class!( "a-z" ) ).apply( &input_state!( "0" ) ).is_none() )
    assert!( and!( lit!( "x" ) ).apply( &input_state!( "y" ) ).is_none() )
  }
}

