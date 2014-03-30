use super::{Expression, ParseState, ParseResult};

macro_rules! plus( ( $ex:expr ) => ( {
    use base;
    base::Plus::new( & $ex ) } ); )

pub struct Plus<'a> {
  expr: &'a Expression
}


impl<'a> Plus<'a> {
  pub fn new( expr: &'a Expression ) -> Plus<'a> {
    Plus { expr: expr }
  }
}


impl<'a> Expression for Plus<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    let mut final_result = ParseResult::fromParseState( *parse_state );
    let mut num_matches = 0;
    loop {
      match self.expr.apply( &final_result.parse_state ) {
        Some( result ) => {
          final_result.parse_state = result.parse_state;
          final_result.nodes.push_all_move( result.nodes );
          num_matches += 1;
        }
        _ => break
      }
    }

    if num_matches > 0 {
      Some( final_result )
    } else {
      None
    }
  }
}


#[cfg(test)]
mod tests {
  use base::{Node, ParseResult, ParseState, Expression, Data};
  use base::literal::{Literal, LITERAL_EXPRESSION};
  use super::{Plus};

  #[test]
  fn Plus_Match() {
    let orig_state = input_state!( "aaa" );
    match plus!( lit!( "a" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 0,
                           end: 1,
                           contents: data!( "a" ) } );
        assert_eq!( *nodes.get( 1 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 1,
                           end: 2,
                           contents: data!( "a" ) } );
        assert_eq!( *nodes.get( 2 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 2,
                           end: 3,
                           contents: data!( "a" ) } );
        assert_eq!( parse_state, orig_state.advanceTo( 3 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn Plus_Match_JustOne() {
    let orig_state = input_state!( "abb" );
    match plus!( lit!( "a" ) ).apply( &orig_state ) {
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
  fn Plus_NoMatch() {
    let orig_state = input_state!( "y" );
    match plus!( lit!( "x" ) ).apply( &orig_state ) {
      None => (),
      _ => fail!( "Should not match." ),
    }
  }
}
