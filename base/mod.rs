#[cfg(not(test))]
use base::literal::LITERAL_EXPRESSION;

pub use self::not::NotEx;
pub use self::and::And;
pub use self::char_class::CharClass;
pub use self::literal::Literal;
pub use self::dot::Dot;
pub use self::option::OptionEx;
pub use self::star::Star;
pub use self::plus::Plus;
pub use self::or::Or;
pub use self::sequence::Sequence;
pub use self::wrap::WrapEx;


#[macro_escape]
pub mod test_utils;

#[macro_escape]
mod literal;
#[macro_escape]
mod char_class;
#[macro_escape]
mod not;
#[macro_escape]
mod and;
mod dot;
#[macro_escape]
mod option;
#[macro_escape]
mod star;
#[macro_escape]
mod plus;
#[macro_escape]
mod or;
#[macro_escape]
mod sequence;
#[macro_escape]
mod wrap;
mod unicode;
mod unescape;


#[deriving(Show, Eq)]
pub enum NodeContents<'a> {
  Data( &'a [u8] ),
  Children( Vec<Node<'a>> ),
  Empty
}


#[deriving(Show, Eq)]
pub struct Node<'a> {
  name: &'static str,
  start: uint,
  end: uint,
  contents: NodeContents<'a>
}

impl<'a> Node<'a> {
  // fn matchedData( &self ) -> Vec<u8> {
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
}


#[deriving(Show, Clone, Eq)]
pub struct ParseState<'a> {
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

pub struct ParseResult<'a> {
  nodes: Vec< Node<'a> >,
  parse_state: ParseState<'a>
}


impl<'a> ParseResult<'a> {
  fn oneNode<'a>( node: Node<'a>, parse_state: ParseState<'a> )
      -> ParseResult<'a> {
    ParseResult { nodes: vec!( node ), parse_state: parse_state }
  }

  // TODO: Needed?
  // fn manyNodes<'a>( nodes: Vec< Node<'a> >, parse_state: ParseState<'a> )
  //     -> ParseResult<'a> {
  //   ParseResult { nodes: nodes, parse_state: parse_state }
  // }

  fn fromParseState<'a>( parse_state: ParseState<'a> ) -> ParseResult<'a> {
    ParseResult { nodes: vec!(), parse_state: parse_state }
  }
}


pub trait Expression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> )
      -> Option< ParseResult<'a> >;
}

type Rule = fn<'a>( &ParseState<'a> ) -> Option< ParseResult<'a> >;


// TODO: We should pass around the lifetime of 'input' to other functions and
// thus avoild allocating Data and stuff
#[cfg(not(test))]
pub fn parseBytes<'a>( input: &'a [u8] ) -> Node<'a> {
  Node { name: LITERAL_EXPRESSION,
         start: 0,
         end: 3,
         contents: Data( input ) }
}
