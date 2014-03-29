use super::{Expression, ParseState, ParseResult};

macro_rules! lit( ( $ex:expr ) => ( {
      byte_var!( input = $ex )
      Literal::new( input )
    } ) )

pub static LITERAL_EXPRESSION : &'static str = "Literal";

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

    parse_state.nameAndOffsetToResult(
          LITERAL_EXPRESSION, parse_state.offset + self.text.len() )
  }
}


#[cfg(test)]
mod tests {
  use base::{Node, Data, ParseResult, ParseState, Expression};
  use base::test_utils::ToParseState;
  use super::{LITERAL_EXPRESSION, Literal};

  #[test]
  fn Literal_Match() {
    let expr = lit!( "foo" );
    match expr.apply( &ToParseState( bytes!( "foobar" ) ) ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ),
                    Node { name: LITERAL_EXPRESSION,
                           start: 0,
                           end: 3,
                           contents: data!( "foo" ) } );
        assert_eq!( parse_state, ParseState{ input: bytes!( "bar" ),
                                             offset: 3 } );
      }
      _ => fail!( "No match!" )
    };
  }


  #[test]
  fn Literal_NoMatch() {
    let expr = lit!( "zoo" );
    assert!( expr.apply( &ToParseState( bytes!( "foobar" ) ) ).is_none() );
    assert!( expr.apply( &ToParseState( bytes!( "" ) ) ).is_none() );
  }
}
