use std::iter::Iterator;
use std::iter::Enumerate;
use std::str::Chars;
use parser::literal::LITERAL_EXPRESSION;

mod literal;
mod not;
mod and;
mod char_class;
mod dot;
// mod option;
// mod star;
// mod plus;
// mod parens;
// mod or;
// mod sequence;
mod helpers;
mod test_utils;


#[deriving(Show, Eq)]
enum NodeContents {
  Text( ~str ),
  Children( ~[ ~Node ] ),
  Empty
}


#[deriving(Show, Eq)]
struct Node {
  name: &'static str,
  start: uint,
  end: uint,
  contents: NodeContents
}

impl Node {
  fn matchedText( &self ) -> ~str {
    match self.contents {
      Empty => ~"",
      Text( ref x ) => x.to_owned(),

      // TODO: recurse through children and collect all text
      Children( _ ) => ~"foo"
    }
  }

  fn predicate( name: &'static str ) -> Node {
    Node { name: name, start: 0, end: 0, contents: Empty }
  }
}


type ParseState<'a> = Enumerate< Chars<'a> >;

struct ParseResult<'a> {
  nodes: ~[ Node ],
  parse_state: ParseState<'a>
}


impl<'a> ParseResult<'a> {
  fn oneNode<'a>( node: Node, parse_state: ParseState<'a> ) -> ParseResult<'a> {
    ParseResult { nodes: ~[ node ], parse_state: parse_state }
  }

  fn manyNodes<'a>( nodes: ~[ Node ], parse_state: ParseState<'a> )
      -> ParseResult<'a> {
    ParseResult { nodes: nodes, parse_state: parse_state }
  }
}


trait Expression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) -> Option< ParseResult<'a> >;
}

fn MoveForward<B, T: Iterator<B> >( mut iter: T, steps: uint ) -> T {
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

