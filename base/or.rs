use super::{Expression, ParseState, ParseResult};

macro_rules! or( ( $( $ex:expr ),* ) => ( {
    use base;
    base::Or::new( &[ $( & $ex as &base::Expression ),* ] ) } ); )

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

  #[test]
  fn Or_Match_FirstExpr() {
    let orig_state = input_state!( b"a" );
    match or!( lit!( b"a" ), lit!( b"b" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node::noName( 0, 1, Data( b"a" ) ) );
        assert_eq!( parse_state, orig_state.advanceTo( 1 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn Or_Match_SecondExpr() {
    let orig_state = input_state!( b"a" );
    match or!( lit!( b"b" ), lit!( b"a" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node::noName( 0, 1, Data( b"a" ) ) );
        assert_eq!( parse_state, orig_state.advanceTo( 1 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn Or_Match_FirstExprIfBoth() {
    let orig_state = input_state!( b"a" );
    match or!( lit!( b"a" ), lit!( b"a" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node::noName( 0, 1, Data( b"a" ) ) );
        assert_eq!( parse_state, orig_state.advanceTo( 1 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn Or_NoMatch() {
    assert!( or!( lit!( b"b" ), lit!( b"c" ) ).apply(
        &input_state!( b"a" ) ).is_none() )
  }
}

