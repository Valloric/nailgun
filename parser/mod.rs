use parser::literal::LITERAL_EXPRESSION;

mod literal;
mod not;
mod and;
mod char_class;
mod dot;
mod option;
// mod star;
// mod plus;
// mod parens;
// mod or;
// mod sequence;
mod unicode;
mod unescape;
mod test_utils;


#[deriving(Show, Eq)]
enum NodeContents<'a> {
  Data( &'a [u8] ),
  Children( ~[ Node<'a> ] ),
  Empty
}


#[deriving(Show, Eq)]
struct Node<'a> {
  name: &'static str,
  start: uint,
  end: uint,
  contents: NodeContents<'a>
}

impl<'a> Node<'a> {
  // fn matchedData( &self ) -> ~[u8] {
  //   match self.contents {
  //     Empty => ~"",
  //     Data( ref x ) => x.to_owned(),
  //
  //     // TODO: recurse through children and collect all text
  //     Children( _ ) => ~"foo"
  //   }
  // }

  // TODO: methods in_order/pre_order/post_order that yield
  // iterators for walking the node tree structure

  fn predicate( name: &'static str ) -> Node {
    Node { name: name, start: 0, end: 0, contents: Empty }
  }
}


#[deriving(Show, Clone, Eq)]
struct ParseState<'a> {
  input: &'a [u8],  // Unconsumed input from "main" slice.
  offset: uint  // Offset of 'input' from start of "main" slice.
}


impl<'a> ParseState<'a> {
  fn advanceTo( &self, new_offset: uint ) -> ParseState<'a> {
    let mut clone = self.clone();
    clone.input = clone.input.slice_from( new_offset - clone.offset );
    clone.offset = new_offset;
    clone
  }

  fn sliceTo( &self, new_offset: uint ) -> &'a [u8] {
    self.input.slice_to( new_offset - self.offset )
  }

  fn nameAndOffsetToResult( &self, node_name: &'static str, new_offset: uint )
      -> Option< ParseResult<'a> > {
    Some( ParseResult::oneNode(
        Node { name: node_name,
               start: self.offset,
               end: new_offset,
               contents: Data( self.sliceTo( new_offset ) ) },
        self.advanceTo( new_offset ) ) )
  }
}

struct ParseResult<'a> {
  nodes: ~[ Node<'a> ],
  parse_state: ParseState<'a>
}


impl<'a> ParseResult<'a> {
  fn oneNode<'a>( node: Node<'a>, parse_state: ParseState<'a> )
      -> ParseResult<'a> {
    ParseResult { nodes: ~[ node ], parse_state: parse_state }
  }

  fn manyNodes<'a>( nodes: ~[ Node<'a> ], parse_state: ParseState<'a> )
      -> ParseResult<'a> {
    ParseResult { nodes: nodes, parse_state: parse_state }
  }
}


trait Expression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) -> Option< ParseResult<'a> >;
}


// TODO: We should pass around the lifetime of 'input' to other functions and
// thus avoild allocating Data and stuff
pub fn parseBytes<'a>( input: &'a [u8] ) -> Node<'a> {
  Node { name: LITERAL_EXPRESSION,
         start: 0,
         end: 3,
         contents: Data( input ) }
}

