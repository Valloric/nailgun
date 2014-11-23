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
use std::fmt;
use std::str;
use std::fmt::{Result};
pub use self::NodeContents::{Data, Children};

static NO_NAME : &'static str = "<none>";

pub struct PreOrderNodes<'a, 'b:'a> {
  queue: Vec<&'a Node<'b>>
}

impl<'a, 'b:'a> Iterator<&'a Node<'b>> for PreOrderNodes<'a, 'b> {
  fn next( &mut self ) -> Option<&'a Node<'b>> {
    match self.queue.pop() {
      Some( node ) => {
        match node.contents {
          Children( ref x ) => {
            for child in x[].iter().rev() {
              self.queue.push( child )
            }
          }
          _ => ()
        };
        Some( node )
      }
      _ => None
    }
  }
}


#[deriving(Show, PartialEq)]
pub enum NodeContents<'a> {
  /// A `&[u8]` byte slice this node matched in the parse input. Only leaf nodes
  /// have `Data` contents.
  Data( &'a [u8] ),

  /// Children of the node, if any. Only non-leaf nodes have `Children`
  /// contents.
  Children( Vec<Node<'a>> )
}


#[deriving(PartialEq)]
pub struct Node<'a> {
  /// The name of the node.
  pub name: &'static str,

  /// The (inclusive) start index of the range this node matches. It's the byte
  /// (NOT char) offset of the parse input.
  pub start: uint,

  /// The (exclusive) end index of the range this node matches. It's the byte
  /// (NOT char) offset of the parse input.
  pub end: uint,

  /// The contents of the node; this can be either children nodes or a matched
  /// `&[u8]` slice.
  pub contents: NodeContents<'a>
}


fn indent( formatter: &mut fmt::Formatter, indent_spaces: int )
    -> fmt::Result {
  for _ in range( 0, indent_spaces ) {
    try!( write!( formatter, " " ) )
  }
  Ok(())
}


impl<'a> Node<'a> {
  fn format( &self, formatter: &mut fmt::Formatter, indent_spaces: int )
      -> fmt::Result {
    try!( indent( formatter, indent_spaces ) );
    try!( write!( formatter,
                  "{0} [{1}, {2}>",
                  self.displayName(), self.start, self.end ) );

    match self.contents {
      Data( data ) => {
        match str::from_utf8( data ) {
          Some( string ) => {
            try!( writeln!( formatter,
                            ": \"{0}\"",
                            string ) );
          }
          _ => {
            try!( writeln!( formatter,
                            ": \"{0}\"",
                            data ) );
          }
        }
      }
      Children( ref children ) => {
        try!( writeln!( formatter, "" ) );
        for child in children.iter() {
          try!( child.format( formatter, indent_spaces + 1) )
        }
      }
    };

    Ok(())
  }

  /// The node name if set, or "<none>" if unset.
  pub fn displayName( &self ) -> &'static str {
    if !self.name.is_empty() {
      self.name
    } else {
      NO_NAME
    }
  }

  /// Creates a `Node` with an empty name.
  pub fn withoutName( start: uint, end: uint, contents: NodeContents<'a> )
      -> Node<'a> {
    Node { name: "", start: start, end: end, contents: contents }
  }

  /// Creates a `Node` with the provided `name` and makes it a parent of the
  /// provided `children`.
  pub fn withChildren( name: &'static str, mut children: Vec<Node<'a>> )
      -> Node<'a> {
    // In case 'children' has only one node with an empty name, our new Node
    // will take the guts of the child with the new 'name'.
    if children.len() == 1 && children[ 0 ].name.is_empty() {
      match children.pop() {
        Some( mut child ) => {
          child.name = name;
          return child;
        }
        _ => ()
      }
    }

    let start = if children.len() != 0 {
      children[ 0 ].start
    } else {
      0
    };

    let end = match children.last() {
      Some( ref node ) => node.end,
      _ => 0
    };

    Node { name: name,
           start: start,
           end: end,
           contents: Children( children ) }
  }


  /// Traverses the tree rooted at the node with pre-order traversal. Includes
  /// the `self` node as the first node.
  #[allow(dead_code)]
  pub fn preOrder<'b>( &'b self ) -> PreOrderNodes<'b, 'a> {
    PreOrderNodes { queue: vec!( self ) }
  }


  /// Concatenates and returns all `&[u8]` data in the leaf nodes beneath
  /// the current node.
  #[allow(dead_code)]
  pub fn matchedData( &self ) -> Vec<u8> {
    match self.contents {
      Data( x ) => x.to_vec(),
      Children( ref children ) => {
        let mut out : Vec<u8> = vec!();
        for child in children.iter() {
          out.push_all( child.matchedData()[] );
        }
        out
      }
    }
  }

  // TODO: methods in_order/pre_order/post_order that yield
  // iterators for walking the node tree structure
}

impl<'a> fmt::Show for Node<'a> {
  fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
    self.format( formatter, 0 )
  }
}


#[cfg(test)]
mod tests {
  use super::{Node, Data};

  fn nameOnly( name: &'static str ) -> Node {
    Node { name: name, start: 0, end: 0, contents: Data( b"" ) }
  }

  fn contentsOnly( contents: &'static [u8] ) -> Node {
    Node { name: "", start: 0, end: 0, contents: Data( contents ) }
  }

  fn testTree() -> Node<'static> {
    // Tree looks like the following:
    //        a
    //   b    c    d
    //  e f   g
    Node::withChildren( "a", vec!(
        Node::withChildren( "b", vec!( nameOnly( "e" ), nameOnly( "f" ) ) ),
        Node::withChildren( "c", vec!( nameOnly( "g" ) ) ),
        nameOnly( "d" ) ) )
  }

  fn testTreeWithContents() -> Node<'static> {
    // Tree looks like the following (nodes with ' have contents):
    //          a
    //    b     c     'd
    // 'e  'f  'g
    Node::withChildren( "a", vec!(
        Node::withChildren(
          "b", vec!( contentsOnly( b"e" ), contentsOnly( b"f" ) ) ),
        Node::withChildren( "c", vec!( contentsOnly( b"g" ) ) ),
        contentsOnly( b"d" ) ) )
  }


  #[test]
  fn preOrder_FullIteration() {
    let root = testTree();
    let names =
      root.preOrder().map( |x| x.name.char_at( 0 ) ).collect::<Vec<_>>();
    assert_eq!( names, vec!( 'a', 'b', 'e', 'f', 'c', 'g', 'd' ) )
  }


  #[test]
  fn matchedData_FullTree() {
    let root = testTreeWithContents();
    assert_eq!( b"efgd", root.matchedData()[] )
  }
}
