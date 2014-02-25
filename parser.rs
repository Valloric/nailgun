enum NodeContents {
  Text( ~str ),
  Children( ~[ ~Node ] )
}


struct Node {
  name: ~str,
  start: uint,
  end: uint,
  contents: NodeContents
}


impl Node {
  fn matchedText( &self ) -> ~str {
    match self.contents {
      Text( ref x ) => x.to_owned(),

      // TODO: recurse through children and collect all text
      Children( _ ) => ~"foo"
    }
  }
}


struct ParseState;


struct ParseResult {
  node: Node,
  parse_state: ParseState
}


trait Expression {
  fn apply( parse_state: ParseState ) -> Option< ParseResult >;
}


struct LiteralExpression {
  text: ~str
}


impl Expression for LiteralExpression {
  fn apply( parse_state: ParseState ) -> Option< ParseResult > {
    None
  }
}


struct DotExpression;
impl Expression for DotExpression {
  fn apply( parse_state: ParseState ) -> Option< ParseResult > {
    None
  }
}


struct NotExpression;
impl Expression for NotExpression {
  fn apply( parse_state: ParseState ) -> Option< ParseResult > {
    None
  }
}


struct CharClassExpression {
  contents: ~str
}


impl CharClassExpression {
  fn new( contents: &str ) -> CharClassExpression {
    CharClassExpression{ contents: contents.to_owned() }  // parse the contents!
  }
}


impl Expression for CharClassExpression {
  fn apply( parse_state: ParseState ) -> Option< ParseResult > {
    None
  }
}


pub fn parseString( input: &str ) -> Node {
  Node { name: ~"foo",
         start: 0,
         end: 3,
         contents: Text( input.to_owned() ) }
}
