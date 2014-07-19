// Copyright 2014 Strahinja Val Markovic
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
pub use self::not::NotEx;
pub use self::and::And;
pub use self::fuse::Fuse;
pub use self::char_class::CharClass;
pub use self::literal::Literal;
pub use self::dot::Dot;
pub use self::option::OptionEx;
pub use self::star::Star;
pub use self::plus::Plus;
pub use self::or::Or;
pub use self::sequence::Sequence;
pub use self::wrap::WrapEx;
pub use self::node::{Node, NodeContents, Data, Children, PreOrderNodes};


mod node;
#[cfg(test)]
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
mod fuse;
#[macro_escape]
mod sequence;
#[macro_escape]
mod wrap;
mod unicode;


#[doc(hidden)]
#[deriving(Show, Clone, PartialEq)]
pub struct ParseState<'a> {
  pub input: &'a [u8],  // Unconsumed input from "main" slice.
  pub offset: uint  // Offset of 'input' from start of "main" slice.
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

  fn offsetToResult( &self, new_offset: uint )
      -> Option< ParseResult<'a> > {
    Some( ParseResult::oneNode(
            Node::withoutName( self.offset,
                               new_offset,
                               Data( self.sliceTo( new_offset ) ) ),
            self.advanceTo( new_offset ) ) )
  }
}

#[doc(hidden)]
pub struct ParseResult<'a> {
  pub nodes: Vec< Node<'a> >,
  pub parse_state: ParseState<'a>
}


impl<'a> ParseResult<'a> {
  pub fn oneNode<'a>( node: Node<'a>, parse_state: ParseState<'a> )
      -> ParseResult<'a> {
    ParseResult { nodes: vec!( node ), parse_state: parse_state }
  }

  pub fn fromParseState<'a>( parse_state: ParseState<'a> ) -> ParseResult<'a> {
    ParseResult { nodes: vec!(), parse_state: parse_state }
  }
}


pub trait Expression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> )
      -> Option< ParseResult<'a> >;
}

type Rule = fn<'a>( &ParseState<'a> ) -> Option< ParseResult<'a> >;

