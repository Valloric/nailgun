use super::{ ParseState, LiteralExpression, LITERAL_EXPRESSION, DotExpression,
DOT_EXPRESSION, Text, ParseResult, Node, Expression, CHAR_CLASS_EXPRESSION,
CharClassExpression, NotExpression, NOT_EXPRESSION };


fn ToParseState<'a>( text: &'a str ) -> ParseState<'a> {
  text.chars().enumerate()
}


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
  assert!( expr.apply( &ToParseState( "foobar" ) ).is_none() )
  assert!( expr.apply( &ToParseState( "" ) ).is_none() )
}


#[test]
fn DotExpression_Match_InputOneChar() {
  match DotExpression.apply( &ToParseState( "x" ) ) {
    Some( ParseResult{ nodes: ref nodes,
                       parse_state: mut parse_state } ) => {
      assert_eq!( *nodes.get( 0 ).unwrap(),
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
    Some( ParseResult{ nodes: ref nodes,
                       parse_state: mut parse_state } ) => {
      assert_eq!( *nodes.get( 0 ).unwrap(),
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
    Some( ParseResult{ nodes: ref nodes,
                       parse_state: mut parse_state } ) => {
      assert_eq!( *nodes.get( 0 ).unwrap(),
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


fn charClassMatch( char_class: CharClassExpression, input: &str ) -> bool {
  match char_class.apply( &ToParseState( input ) ) {
    Some( ParseResult{ nodes: ref nodes,
                       parse_state: mut parse_state } ) => {
      assert_eq!( *nodes.get( 0 ).unwrap(),
                  Node { name: CHAR_CLASS_EXPRESSION,
                         start: 0,
                         end: 1,
                         contents: Text( input.to_owned() ) } );
      assert_eq!( parse_state.next(), None );
      true
    }
    _ => false
  }
}


#[test]
fn CharClassExpression_Match() {
  assert!( charClassMatch( CharClassExpression::new( "a"          ), "a" ) );
  assert!( charClassMatch( CharClassExpression::new( "abcdef"     ), "e" ) );
  assert!( charClassMatch( CharClassExpression::new( "a-z"        ), "a" ) );
  assert!( charClassMatch( CharClassExpression::new( "a-z"        ), "c" ) );
  assert!( charClassMatch( CharClassExpression::new( "a-z"        ), "z" ) );
  assert!( charClassMatch( CharClassExpression::new( "0-9"        ), "2" ) );
  assert!( charClassMatch( CharClassExpression::new( "α-ω"        ), "η" ) );
  assert!( charClassMatch( CharClassExpression::new( "-"          ), "-" ) );
  assert!( charClassMatch( CharClassExpression::new( "a-"         ), "-" ) );
  assert!( charClassMatch( CharClassExpression::new( "-a"         ), "-" ) );
  assert!( charClassMatch( CharClassExpression::new( "a-zA-Z-"    ), "-" ) );
  assert!( charClassMatch( CharClassExpression::new( "aa-zA-Z-a"  ), "-" ) );
  assert!( charClassMatch( CharClassExpression::new( "a-zA-Z-"    ), "z" ) );
  assert!( charClassMatch( CharClassExpression::new( "aa-zA-Z-0"  ), "0" ) );
  assert!( charClassMatch( CharClassExpression::new( "a-cdefgh-k" ), "e" ) );
  assert!( charClassMatch( CharClassExpression::new( "---"        ), "-" ) );
  assert!( charClassMatch( CharClassExpression::new( "a-a"        ), "a" ) );
}


#[test]
fn CharClassExpression_NoMatch() {
  assert!( !charClassMatch( CharClassExpression::new( "a"   ), "b" ) );
  assert!( !charClassMatch( CharClassExpression::new( "-"   ), "a" ) );
  assert!( !charClassMatch( CharClassExpression::new( "z-a" ), "a" ) );
  assert!( !charClassMatch( CharClassExpression::new( "z-a" ), "b" ) );
  assert!( !charClassMatch( CharClassExpression::new( "a-z" ), "0" ) );
  assert!( !charClassMatch( CharClassExpression::new( "a-z" ), "A" ) );
}


#[test]
fn NotExpression_Match_WithLiteral() {
  match NotExpression::new( ~LiteralExpression::new( "foo" ) ).apply(
      &ToParseState( "zoo" ) ) {
    Some( ParseResult{ nodes: ref nodes,
                       parse_state: mut parse_state } ) => {
      assert_eq!( *nodes.get( 0 ).unwrap(), Node::predicate( NOT_EXPRESSION ) );
      assert_eq!( parse_state.next(), Some( ( 0, 'z' ) ) );
    }
    _ => fail!( "No match." )
  }

  assert!( NotExpression::new( ~CharClassExpression::new( "a-z" ) )
           .apply( &ToParseState( "0" ) ).is_some() )
}


#[test]
fn NotExpression_Match_WithCharClass() {
  match NotExpression::new( ~CharClassExpression::new( "a-z" ) ).apply(
      &ToParseState( "0" ) ) {
    Some( ParseResult{ nodes: ref nodes,
                       parse_state: mut parse_state } ) => {
      assert_eq!( *nodes.get( 0 ).unwrap(), Node::predicate( NOT_EXPRESSION ) );
      assert_eq!( parse_state.next(), Some( ( 0, '0' ) ) );
    }
    _ => fail!( "No match." )
  }
}


#[test]
fn NotExpression_NoMatch() {
  assert!( NotExpression::new( ~CharClassExpression::new( "a-z" ) ).apply(
      &ToParseState( "b" ) ).is_none() )
  assert!( NotExpression::new( ~LiteralExpression::new( "x" ) ).apply(
      &ToParseState( "x" ) ).is_none() )
}
