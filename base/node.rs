use std::fmt;
use std::str;
use std::fmt::{Result};

static EMPTY : &'static str = "";
static NO_NAME : &'static str = "<none>";

#[deriving(Show, Eq)]
pub enum NodeContents<'a> {
  /// A `&[u8]` byte slice this node matched in the parse input. Only leaf nodes
  /// have `Data` contents.
  Data( &'a [u8] ),

  /// Children of the node, if any. Only non-leaf nodes have `Children`
  /// contents.
  Children( Vec<Node<'a>> )
}


#[deriving(Eq)]
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
    try!( write!( formatter.buf, " " ) )
  }
  Ok(())
}

impl<'a> Node<'a> {
  fn format( &self, formatter: &mut fmt::Formatter, indent_spaces: int )
      -> fmt::Result {
    try!( indent( formatter, indent_spaces ) );
    try!( write!( formatter.buf,
                  "Node \\{name: {0}, start: {1}, end: {2}",
                  self.displayName(), self.start, self.end ) );

    match self.contents {
      Data( data ) => {
        match str::from_utf8( data ) {
          Some( string ) => {
            try!( writeln!( formatter.buf,
                            ", contents: \"{0}\" \\}",
                            string ) );
          }
          _ => {
            try!( writeln!( formatter.buf,
                            ", contents: \"{0}\" \\}",
                            data ) );
          }
        }
      }
      Children( ref children ) => {
        try!( writeln!( formatter.buf, " \\}" ) );
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
  pub fn noName( start: uint, end: uint, contents: NodeContents<'a> )
      -> Node<'a> {
    Node { name: EMPTY, start: start, end: end, contents: contents }
  }

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


