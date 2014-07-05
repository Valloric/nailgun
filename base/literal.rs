use super::{Expression, ParseState, ParseResult};

macro_rules! lit( ( $ex:expr ) => ( {
      use base;
      base::Literal::new( $ex )
    } ) )


pub struct Literal {
  text: &'static [u8]
}


impl Literal {
  pub fn new( text: &'static [u8] ) -> Literal {
    Literal { text: text }
  }
}


impl Expression for Literal {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    if parse_state.input.len() < self.text.len() ||
       parse_state.input.slice_to( self.text.len() ) != self.text {
      return None;
    }

    parse_state.offsetToResult( parse_state.offset + self.text.len() )
  }
}


#[cfg(test)]
mod tests {
  use base::{Node, Data, ParseResult, ParseState, Expression};

  #[test]
  fn Literal_Match() {
    let expr = lit!( b"foo" );
    match expr.apply( &input_state!( b"foobar" ) ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node::noName( 0, 3, Data( b"foo" ) ) );
        assert_eq!( parse_state, ParseState{ input: b"bar",
                                             offset: 3 } );
      }
      _ => fail!( "No match!" )
    };
  }


  #[test]
  fn Literal_NoMatch() {
    let expr = lit!( b"zoo" );
    assert!( expr.apply( &input_state!( b"foobar" ) ).is_none() );
    assert!( expr.apply( &input_state!( b"" ) ).is_none() );
  }
}
