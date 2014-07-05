use super::{Expression, ParseState, ParseResult};

macro_rules! star( ( $ex:expr ) => ( {
    use base;
    base::Star::new( & $ex ) } ); )

pub struct Star<'a> {
  expr: &'a Expression
}


impl<'a> Star<'a> {
  pub fn new<'a>( expr: &'a Expression ) -> Star<'a> {
    Star { expr: expr }
  }
}


impl<'a> Expression for Star<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    let mut final_result = ParseResult::fromParseState( *parse_state );
    loop {
      match self.expr.apply( &final_result.parse_state ) {
        Some( result ) => {
          final_result.parse_state = result.parse_state;
          final_result.nodes.push_all_move( result.nodes );
        }
        _ => break
      }
    }
    Some( final_result )
  }
}


#[cfg(test)]
mod tests {
  use base::{Node, ParseResult, Expression, Data};

  #[test]
  fn Star_Match() {
    let orig_state = input_state!( b"aaa" );
    match star!( lit!( b"a" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node::noName( 0, 1, Data( b"a" ) ) );
        assert_eq!( *nodes.get( 1 ),
                    Node::noName( 1, 2, Data( b"a" ) ) );
        assert_eq!( *nodes.get( 2 ),
                    Node::noName( 2, 3, Data( b"a" ) ) );
        assert_eq!( parse_state, orig_state.advanceTo( 3 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn Star_Match_JustOne() {
    let orig_state = input_state!( b"abb" );
    match star!( lit!( b"a" ) ).apply( &orig_state ) {
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
  fn Star_Match_Empty() {
    let orig_state = input_state!( b"y" );
    match star!( lit!( b"x" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert!( nodes.is_empty() );
        assert_eq!( parse_state, orig_state );
      }
      _ => fail!( "No match." )
    }
  }
}
