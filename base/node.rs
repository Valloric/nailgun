use std::fmt;
use std::str;
use std::fmt::{Result};

static EMPTY : &'static str = "";

#[deriving(Show, Eq)]
pub enum NodeContents<'a> {
  Data( &'a [u8] ),
  Children( Vec<Node<'a>> )
}


#[deriving(Eq)]
pub struct Node<'a> {
  pub name: &'static str,
  pub start: uint,
  pub end: uint,
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
                  self.name, self.start, self.end ) );

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

  pub fn noName( start: uint, end: uint, contents: NodeContents<'a> )
      -> Node<'a> {
    Node { name: EMPTY, start: start, end: end, contents: contents }
  }

  pub fn newParent( name: &'static str, children: Vec<Node<'a>> )
      -> Node<'a> {
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


