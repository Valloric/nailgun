use std::fmt;
use std::str;
use std::fmt::{Result};

static NO_NAME : &'static str = "<none>";

// NOTE: Uncomment when https://github.com/mozilla/rust/issues/13703 is fixed.
// pub struct PreOrderNodes<'a, 'b> {
//   queue: Vec<&'a Node<'b>>
// }
//
// impl<'a, 'b> Iterator<&'a Node<'b>> for PreOrderNodes<'a, 'b> {
//   fn next( &mut self ) -> Option<&'a Node<'b>> {
//     match self.queue.pop() {
//       ex @ Some( node ) => {
//         match node.contents {
//           Children( ref x ) => {
//             for child in x.as_slice().iter().rev() {
//               self.queue.push( child )
//             }
//           }
//           _ => ()
//         };
//         ex
//       }
//       _ => None
//     }
//   }
// }


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
                  "Node {{name: {0}, start: {1}, end: {2}",
                  self.displayName(), self.start, self.end ) );

    match self.contents {
      Data( data ) => {
        match str::from_utf8( data ) {
          Some( string ) => {
            try!( writeln!( formatter,
                            ", contents: \"{0}\" }}",
                            string ) );
          }
          _ => {
            try!( writeln!( formatter,
                            ", contents: \"{0}\" }}",
                            data ) );
          }
        }
      }
      Children( ref children ) => {
        try!( writeln!( formatter, " }}" ) );
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

  // TODO: rename this to withChildren?
  /// Creates a `Node` with the provided `name` and makes it a parent of the
  /// provided `children`.
  pub fn newParent( name: &'static str, mut children: Vec<Node<'a>> )
      -> Node<'a> {
    // In case 'children' has only one node with an empty name, our new Node
    // will take the guts of the child with the new 'name'.
    if children.len() == 1 && children.get( 0 ).name.is_empty() {
      match children.pop() {
        Some( mut child ) => {
          child.name = name;
          return child;
        }
        _ => ()
      }
    }

    let start = if children.len() != 0 {
      children.get( 0 ).start
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


  /// Traverses the tree rooted at the node with pre-order traversal. `visitor`
  /// is called on every node and traversal stops when `visitor` returns `false`.
  ///
  /// Normally this function would return an iterator instead of taking a
  /// visitor function, but a `rustc` bug is preventing that implementation.
  #[allow(dead_code)]
  pub fn preOrder( &self, visitor: |&Node| -> bool ) {
    fn inner( node: &Node, visitor: |&Node| -> bool ) -> bool {
      if !visitor( node ) {
        return false;
      }

      match node.contents {
        Children( ref x ) => {
          for node in x.iter() {
            if !inner( node, |x| visitor( x ) ) {
              return false;
            }
          }
        }
        _ => ()
      };

      return true;
    }
    inner( self, visitor );
  }

  // NOTE: Uncomment when https://github.com/mozilla/rust/issues/13703 is fixed.
  // pub fn preOrder<'b>( &'b self ) -> PreOrderNodes<'b, 'a> {
  //   PreOrderNodes { queue: vec!( self ) }
  // }

  #[allow(dead_code)]
  fn matchedData( &self ) -> Vec<u8> {
    match self.contents {
      Data( x ) => Vec::from_slice( x ),
      Children( _ ) => {
        // TODO: implement
        vec!()
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


  fn testTree() -> Node {
    // Tree looks like the following:
    //        a
    //   b    c    d
    //  e f   g
    Node::newParent( "a", vec!(
        Node::newParent( "b", vec!( nameOnly( "e" ), nameOnly( "f" ) ) ),
        Node::newParent( "c", vec!( nameOnly( "g" ) ) ),
        nameOnly( "d" ) ) )
  }


  #[test]
  fn preOrder_FullIteration() {
    let root = testTree();
    let mut names : Vec<char> = vec!();
    root.preOrder( |ref x| {
      names.push( x.name.char_at( 0 ) );
      true
    });

    assert_eq!( names, vec!( 'a', 'b', 'e', 'f', 'c', 'g', 'd' ) )
  }


  #[test]
  fn preOrder_PartialIteration() {
    let root = testTree();
    let mut names : Vec<char> = vec!();
    root.preOrder( |ref x| {
      let ch =  x.name.char_at( 0 );
      names.push( ch );
      if ch == 'f' { false } else { true }
    });

    assert_eq!( names, vec!( 'a', 'b', 'e', 'f' ) )
  }
}
