use super::{Expression, Node, Data, ParseState, ParseResult};

pub static LITERAL_EXPRESSION : &'static str = "LiteralExpression";

pub struct LiteralExpression {
  text: &'static [u8]
}


impl LiteralExpression {
  pub fn new( text: &'static [u8] ) -> LiteralExpression {
    LiteralExpression { text: text }
  }
}


impl Expression for LiteralExpression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    if parse_state.input.len() < self.text.len() ||
       parse_state.input.slice_to( self.text.len() ) != self.text {
      return None;
    }

    let new_offset = parse_state.offset + self.text.len();
    Some( ParseResult::oneNode(
        Node { name: LITERAL_EXPRESSION,
               start: parse_state.offset,
               end: new_offset,
               contents: Data( parse_state.sliceTo( new_offset ) ) },
        parse_state.advanceTo( new_offset ) ) )
  }
}


#[cfg(test)]
mod tests {
  use parser::{Node, Data, ParseResult, ParseState, Expression};
  use parser::test_utils::ToParseState;
  use super::{LITERAL_EXPRESSION, LiteralExpression};

  #[test]
  fn LiteralExpression_Match() {
    static literal: &'static [u8] = bytes!( "foo" );
    let expr = LiteralExpression::new( literal );
    match expr.apply( &ToParseState( bytes!( "foobar" ) ) ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( *nodes.get( 0 ).unwrap(),
                    Node { name: LITERAL_EXPRESSION,
                           start: 0,
                           end: 3,
                           contents: Data( literal ) } );
        assert_eq!( parse_state, ParseState{ input: bytes!( "bar" ),
                                             offset: 3 } );
      }
      _ => fail!( "No match!" )
    };
  }


  #[test]
  fn LiteralExpression_NoMatch() {
    static literal: &'static [u8] = bytes!( "zoo" );
    let expr = LiteralExpression::new( literal );
    assert!( expr.apply( &ToParseState( bytes!( "foobar" ) ) ).is_none() );
    assert!( expr.apply( &ToParseState( bytes!( "" ) ) ).is_none() );
  }
}
