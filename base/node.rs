use std::fmt;
use std::str;
use std::fmt::{Result};

#[deriving(Show, Eq)]
pub enum NodeContents<'a> {
  Data( &'a [u8] ),
  Children( Vec<Node<'a>> ),
  Empty
}


#[deriving(Eq)]
pub struct Node<'a> {
  name: &'static str,
  start: uint,
  end: uint,
  contents: NodeContents<'a>
}

impl<'a> Node<'a> {
  fn format( &self, formatter: &mut fmt::Formatter, indent_spaces: int )
      -> fmt::Result {
    fn indent( formatter: &mut fmt::Formatter, indent_spaces: int )
        -> fmt::Result {
      for _ in range( 0, indent_spaces ) {
        try!( write!( formatter.buf, " " ) )
      }
      Ok(())
    }

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
      _ => ()
    };

    Ok(())
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

impl<'a> fmt::Show for Node<'a> {
  fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
    self.format( formatter, 0 )
  }
}


