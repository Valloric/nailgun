use std::str::Chars;
use std::str::from_char;
use std::iter::Iterator;
use std::iter::Enumerate;

static LITERAL_EXPRESSION : &'static str = "LiteralExpression";
static DOT_EXPRESSION : &'static str = "DotExpression";

#[deriving(Eq)]
enum NodeContents {
  Text( ~str ),
  Children( ~[ ~Node ] )
}


#[deriving(Eq)]
struct Node {
  name: &'static str,
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


type ParseState<'a> = Enumerate< Chars<'a> >;

struct ParseResult<'a> {
  node: Option< Node >,
  parse_state: ParseState<'a>
}


trait Expression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) -> Option< ParseResult<'a> >;
}


struct LiteralExpression {
  text: &'static str
}


impl Expression for LiteralExpression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    let indices_and_chars =
      parse_state.take( self.text.len() ).collect::< ~[ ( uint, char ) ] >();
    let chars : ~str =
      indices_and_chars.iter().map( | &( _, ch ) | ch ).collect();

    if self.text == chars {
      Some( ParseResult { parse_state: Increment( parse_state.clone(),
                                                  self.text.len() ),
                          node: Some( Node {
                            name: LITERAL_EXPRESSION,
                            start: indices_and_chars.head().unwrap().val0(),
                            end: indices_and_chars.last().unwrap().val0() + 1,
                            contents: Text( self.text.to_owned() ) } ) } )
    } else {
      None
    }
  }
}


struct DotExpression;
impl Expression for DotExpression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) -> Option< ParseResult<'a> > {
    let mut new_parse_state = parse_state.clone();
    match new_parse_state.next() {
      Some( ( index, character ) ) => Some(
        ParseResult { parse_state: new_parse_state,
                      node: Some( Node {
                        name: DOT_EXPRESSION,
                        start: index,
                        end: index + 1,
                        contents: Text( from_char( character ) ) } ) } ),
      _ => None
    }
  }
}


struct NotExpression;
impl Expression for NotExpression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) -> Option< ParseResult<'a> > {
    None
  }
}


struct CharClassExpression {
  contents: &'static str
}


// impl CharClassExpression {
//   fn new( contents: &str ) -> CharClassExpression {
//     CharClassExpression{ contents: contents.to_owned() }  // parse the contents!
//   }
// }


impl Expression for CharClassExpression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) -> Option< ParseResult<'a> > {
    None
  }
}


fn Increment<B, T: Iterator<B> >( mut iter: T, steps: uint ) -> T {
  for i in range( 0, steps ) {
    iter.next();
  }
  iter
}

pub fn parseString( input: &str ) -> Node {
  Node { name: LITERAL_EXPRESSION,
         start: 0,
         end: 3,
         contents: Text( input.to_owned() ) }
}


#[cfg(test)]
mod parser_tests;
