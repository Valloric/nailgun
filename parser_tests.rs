use super::{ ParseState, LiteralExpression, LITERAL_EXPRESSION, DotExpression,
DOT_EXPRESSION, Text, ParseResult, Node, Expression};


fn ToParseState<'a>( text: &'a str ) -> ParseState<'a> {
  text.chars().enumerate()
}


#[test]
fn LiteralExpression_Match() {
  let expr = LiteralExpression { text: "foo" };
  match expr.apply( &ToParseState( "foobar" ) ) {
    Some( ParseResult{ node: Some( ref node ),
                       parse_state: mut parse_state } ) => {
      assert_eq!( *node,
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
  let expr = LiteralExpression { text: "zoo" };
  assert!( expr.apply( &ToParseState( "foobar" ) ).is_none() )
  assert!( expr.apply( &ToParseState( "" ) ).is_none() )
}


#[test]
fn DotExpression_Match_InputOneChar() {
  match DotExpression.apply( &ToParseState( "x" ) ) {
    Some( ParseResult{ node: Some( ref node ),
                       parse_state: mut parse_state } ) => {
      assert_eq!( *node,
                  Node { name: DOT_EXPRESSION,
                         start: 0,
                         end: 1,
                         contents: Text( ~"x" ) } );
      assert_eq!( parse_state.next(), None );
    }
    _ => fail!( "No match!" )
  };
}


#[test]
fn DotExpression_Match_InputOneWideChar() {
  match DotExpression.apply( &ToParseState( "葉" ) ) {
    Some( ParseResult{ node: Some( ref node ),
                       parse_state: mut parse_state } ) => {
      assert_eq!( *node,
                  Node { name: DOT_EXPRESSION,
                         start: 0,
                         end: 1,
                         contents: Text( ~"葉" ) } );
      assert_eq!( parse_state.next(), None );
    }
    _ => fail!( "No match!" )
  };
}


#[test]
fn DotExpression_Match_InputSeveralChars() {
  match DotExpression.apply( &ToParseState( "xb" ) ) {
    Some( ParseResult{ node: Some( ref node ),
                       parse_state: mut parse_state } ) => {
      assert_eq!( *node,
                  Node { name: DOT_EXPRESSION,
                         start: 0,
                         end: 1,
                         contents: Text( ~"x" ) } );
      assert_eq!( parse_state.next(), Some( ( 1, 'b' ) ) );
    }
    _ => fail!( "No match!" )
  };
}


#[test]
fn DotExpression_NoMatch() {
  assert!( DotExpression.apply( &ToParseState( "" ) ).is_none() )
}
