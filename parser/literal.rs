use super::{Expression, Node, Text, MoveForward, ParseState, ParseResult};

pub static LITERAL_EXPRESSION : &'static str = "LiteralExpression";

pub struct LiteralExpression {
  text: &'static str
}


impl LiteralExpression {
  pub fn new( text: &'static str ) -> LiteralExpression {
    LiteralExpression { text: text }
  }
}


impl Expression for LiteralExpression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    let indices_and_chars =
      parse_state.take( self.text.len() ).collect::< ~[ ( uint, char ) ] >();
    let chars : ~str =
      indices_and_chars.iter().map( | &( _, ch ) | ch ).collect();

    if self.text == chars {
      Some( ParseResult::oneNode(
          Node { name: LITERAL_EXPRESSION,
                 start: indices_and_chars.head().unwrap().val0(),
                 end: indices_and_chars.last().unwrap().val0() + 1,
                 contents: Text( self.text.to_owned() ) },
          MoveForward( parse_state.clone(), self.text.len() ) ) )
    } else {
      None
    }
  }
}


#[cfg(test)]
mod tests {
  use parser::{Node, Text, ParseResult, Expression};
  use parser::test_utils::ToParseState;
  use super::{LITERAL_EXPRESSION, LiteralExpression};

  #[test]
  fn LiteralExpression_Match() {
    let expr = LiteralExpression::new( "foo" );
    match expr.apply( &ToParseState( "foobar" ) ) {
      Some( ParseResult{ nodes: ref nodes,
                         parse_state: mut parse_state } ) => {
        assert_eq!( *nodes.get( 0 ).unwrap(),
                    Node { name: LITERAL_EXPRESSION,
                           start: 0,
                           end: 3,
                           contents: Text( ~"foo" ) } );
        assert_eq!( parse_state.next(), Some( ( 3, 'b' ) ) );
      }
      _ => fail!( "No match!" )
    };
  }


  #[test]
  fn LiteralExpression_NoMatch() {
    let expr = LiteralExpression::new( "zoo" );
    assert!( expr.apply( &ToParseState( "foobar" ) ).is_none() );
    assert!( expr.apply( &ToParseState( "" ) ).is_none() );
  }
}
