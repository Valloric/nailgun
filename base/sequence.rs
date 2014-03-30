use super::{Expression, ParseState, ParseResult};

macro_rules! seq( ( $( $ex:expr ),* ) => ( {
    use base;
    base::Sequence::new( &[ $( & $ex as &base::Expression ),* ] ) } ); )

pub struct Sequence<'a> {
  exprs: &'a [&'a Expression]
}


impl<'a> Sequence<'a> {
  pub fn new<'a>( exprs: &'a [&Expression] ) -> Sequence<'a> {
    Sequence { exprs: exprs }
  }
}


impl<'a> Expression for Sequence<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    let mut final_result = ParseResult::fromParseState( *parse_state );
    for expr in self.exprs.iter() {
      match expr.apply( &final_result.parse_state ) {
        Some( result ) => {
          final_result.parse_state = result.parse_state;
          final_result.nodes.push_all_move( result.nodes );
        }
        _ => return None
      }
    }
    Some( final_result )
  }
}


#[cfg(test)]
mod tests {
  use base::{Node, ParseResult, ParseState, Expression, Data};
  use base::literal::{Literal, LITERAL_EXPRESSION};
  use super::{Sequence};

  #[test]
  fn Sequence_Match() {
    let orig_state = input_state!( "ab" );
    match seq!( lit!( "a" ), lit!( "b" ) ).apply( &orig_state ) {
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
                           contents: data!( "b" ) } );
        assert_eq!( parse_state, orig_state.advanceTo( 2 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn Sequence_NoMatch() {
    assert!( seq!( lit!( "a" ), lit!( "b" ) ).apply(
        &input_state!( "aa" ) ).is_none() )
  }
}
