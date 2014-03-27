use super::{Expression, ParseState, ParseResult};

pub struct PlusExpression<'a> {
  expr: &'a Expression
}


impl<'a> PlusExpression<'a> {
  pub fn new( expr: &'a Expression ) -> PlusExpression<'a> {
    PlusExpression { expr: expr }
  }
}


impl<'a> Expression for PlusExpression<'a> {
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
  use parser::{Node, ParseResult, Expression, Data};
  use parser::literal::{LiteralExpression, LITERAL_EXPRESSION};
  use parser::test_utils::ToParseState;
  use super::{PlusExpression};

  #[test]
  fn PlusExpression_Match() {
    byte_var!(input = "aaa");
    byte_var!(literal = "a");
    let orig_state = ToParseState( input );
    match PlusExpression::new( &LiteralExpression::new( literal ) ).apply(
        &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 0,
                           end: 1,
                           contents: Data( literal ) } );
        assert_eq!( *nodes.get( 1 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 1,
                           end: 2,
                           contents: Data( literal ) } );
        assert_eq!( *nodes.get( 2 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 2,
                           end: 3,
                           contents: Data( literal ) } );
        assert_eq!( parse_state, orig_state.advanceTo( 3 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn PlusExpression_Match_JustOne() {
    byte_var!(input = "abb");
    byte_var!(literal = "a");
    let orig_state = ToParseState( input );
    match PlusExpression::new( &LiteralExpression::new( literal ) ).apply(
        &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 0,
                           end: 1,
                           contents: Data( literal ) } );
        assert_eq!( parse_state, orig_state.advanceTo( 1 ) );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn PlusExpression_NoMatch() {
    byte_var!(input = "y");
    byte_var!(literal = "x");
    let orig_state = ToParseState( input );
    match PlusExpression::new( &LiteralExpression::new( literal ) ).apply(
        &orig_state ) {
      None => (),
      _ => fail!( "Should not match." ),
    }
  }
}

