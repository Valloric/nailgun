use super::{Expression, ParseState, ParseResult};

macro_rules! option( ( $ex:expr ) => ( OptionEx::new( $ex ) ); )

pub struct OptionEx<'a> {
  expr: &'a Expression
}


impl<'a> OptionEx<'a> {
  pub fn new( expr: &'a Expression ) -> OptionEx<'a> {
    OptionEx { expr: expr }
  }
}


impl<'a> Expression for OptionEx<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    match self.expr.apply( parse_state ) {
      result @ Some( _ ) => result,
      _ => Some( ParseResult::fromParseState( *parse_state ) )
    }
  }
}


#[cfg(test)]
mod tests {
  use base::{Node, ParseResult, Expression, Data};
  use base::literal::{Literal, LITERAL_EXPRESSION};
  use base::test_utils::ToParseState;
  use super::{OptionEx};

  #[test]
  fn OptionEx_Match_WithLiteral() {
    byte_var!(input = "foo");
    let orig_state = ToParseState( input );
    match option!( &lit!( "foo" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 0,
                           end: 3,
                           contents: data!( "foo" ) } );
        assert_eq!( parse_state, orig_state.advanceTo( 3 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn OptionEx_Match_Empty() {
    byte_var!(input = "y");
    let orig_state = ToParseState( input );
    match option!( &lit!( "x" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert!( nodes.is_empty() );
        assert_eq!( parse_state, orig_state );
      }
      _ => fail!( "No match." )
    }
  }
}


