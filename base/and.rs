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
    let orig_state = input_state!( b"foo" );
    match and!( lit!( b"foo" ) ).apply( &orig_state ) {
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
    let orig_state = input_state!( b"c" );
    match and!( class!( b"a-z" ) ).apply( &orig_state ) {
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
    assert!( and!( class!( b"a-z" ) ).apply( &input_state!( b"0" ) ).is_none() )
    assert!( and!( lit!( b"x" ) ).apply( &input_state!( b"y" ) ).is_none() )
  }
}

