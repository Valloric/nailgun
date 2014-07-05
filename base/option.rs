use super::{Expression, ParseState, ParseResult};

macro_rules! opt( ( $ex:expr ) => ( {
    use base;
    base::OptionEx::new( & $ex ) } ); )

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

  #[test]
  fn OptionEx_Match_WithLiteral() {
    let orig_state = input_state!( "foo" );
    match opt!( lit!( "foo" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node::noName( 0, 3, Data( b"foo" ) ) );
        assert_eq!( parse_state, orig_state.advanceTo( 3 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn OptionEx_Match_Empty() {
    let orig_state = input_state!( "y" );
    match opt!( lit!( "x" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert!( nodes.is_empty() );
        assert_eq!( parse_state, orig_state );
      }
      _ => fail!( "No match." )
    }
  }
}


