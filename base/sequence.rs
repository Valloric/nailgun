use super::{Expression, ParseState, ParseResult};

macro_rules! seq( ( $( $ex:expr ),* ) => ( Sequence::new( &[ $( $ex ),* ] ) ); )

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
  use base::{Node, ParseResult, Expression, Data};
  use base::literal::{Literal, LITERAL_EXPRESSION};
  use base::test_utils::ToParseState;
  use super::{Sequence};

  #[test]
  fn Sequence_Match() {
    byte_var!(input = "ab");
    byte_var!(literal1 = "a");
    byte_var!(literal2 = "b");
    let orig_state = ToParseState( input );
    match seq!(
      &Literal::new( literal1 ) as &Expression,
      &Literal::new( literal2 ) as &Expression ).apply(
          &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 0,
                           end: 1,
                           contents: Data( literal1 ) } );
        assert_eq!( *nodes.get( 1 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 1,
                           end: 2,
                           contents: Data( literal2 ) } );
        assert_eq!( parse_state, orig_state.advanceTo( 2 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn Sequence_NoMatch() {
    byte_var!(input = "aa");
    byte_var!(literal1 = "a");
    byte_var!(literal2 = "b");
    let orig_state = ToParseState( input );

    assert!( seq!(
      &Literal::new( literal1 ) as &Expression,
      &Literal::new( literal2 ) as &Expression ).apply(
          &orig_state ).is_none() )
  }
}
