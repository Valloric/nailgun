use super::{Expression, ParseState, ParseResult};

macro_rules! or( ( $( $ex:expr ),* ) => (
    Or::new( &[ $( & $ex as &Expression ),* ] ) ); )

pub struct Or<'a> {
  exprs: &'a [&'a Expression]
}


impl<'a> Or<'a> {
  pub fn new<'a>( exprs: &'a [&Expression] ) -> Or<'a> {
    Or { exprs: exprs }
  }
}


impl<'a> Expression for Or<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    for expr in self.exprs.iter() {
      match expr.apply( parse_state ) {
        result @ Some( _ ) => return result,
        _ => ()
      }
    }
    None
  }
}


#[cfg(test)]
mod tests {
  use base::{Node, ParseResult, Expression, Data};
  use base::literal::{Literal, LITERAL_EXPRESSION};
  use base::test_utils::ToParseState;
  use super::{Or};

  #[test]
  fn Or_Match_FirstExpr() {
    byte_var!(input = "a");
    let orig_state = ToParseState( input );
    match or!( lit!( "a" ), lit!( "b" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 0,
                           end: 1,
                           contents: data!( "a" ) } );
        assert_eq!( parse_state, orig_state.advanceTo( 1 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn Or_Match_SecondExpr() {
    byte_var!(input = "a");
    let orig_state = ToParseState( input );
    match or!( lit!( "b" ), lit!( "a" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 0,
                           end: 1,
                           contents: data!( "a" ) } );
        assert_eq!( parse_state, orig_state.advanceTo( 1 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn Or_Match_FirstExprIfBoth() {
    byte_var!(input = "a");
    let orig_state = ToParseState( input );
    match or!( lit!( "a" ), lit!( "a" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 0,
                           end: 1,
                           contents: data!( "a" ) } );
        assert_eq!( parse_state, orig_state.advanceTo( 1 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn Or_NoMatch() {
    byte_var!(input = "a");
    let orig_state = ToParseState( input );

    assert!( or!( lit!( "b" ), lit!( "c" ) ).apply( &orig_state ).is_none() )
  }
}

