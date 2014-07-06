use std::str;
use inlined_parser::{Node, Children, Data};
use self::unescape::unescapeString;

mod unescape;

// TODO: figure out how to write this as a function on Node; the borrow checker
// was extra painful the last time you tried.
macro_rules! node_children( ( $node:expr ) => ( {
  match $node.contents {
    Children( ref nodes ) => nodes,
    _ => fail!( "No children in node." )
  } } ) )


pub fn codeForNode( node: &Node ) -> String {
  match node.name {
    "Definition" => wrapChildrenOutput( "rule!( ", node, " )\n" ),
    "Expression" => expressionOutput( node ),
    "Sequence" => sequenceOutput( node ),
    "Literal" => literalOutput( node ),
    "Class" => classOutput( node ),
    "Suffix" => suffixOutput( node ),
    "Prefix" => prefixOutput( node ),
    "Primary" => primaryOutput( node ),
    "DOT" => String::from_str( "base::Dot" ),
    "LEFTARROW" => String::from_str( " <- " ),
    "SLASH" => String::from_str( ", " ),
    "Spacing" => String::new(),
    "EndOfLine" => String::new(),
    "OPEN" => String::new(),
    "CLOSE" => String::new(),
    _ => codeForNodeContents( node )
  }
}


fn codeForNodeContents( node: &Node ) -> String {
  match node.contents {
    Children( ref children ) => {
      children.iter().map( codeForNode ).collect::<Vec<String>>().concat()
    }
    Data( data ) => str::from_utf8( data ).unwrap().to_string(),
  }
}


fn wrapChildrenOutput( before: &str, node: &Node, after: &str ) -> String {
  [ before.into_maybe_owned(),
    codeForNodeContents( node ).into_maybe_owned(),
    after.into_maybe_owned() ].concat()
}


fn wrapNodeOutput( before: &str, node: &Node, after: &str ) -> String {
  [ before.into_maybe_owned(),
    codeForNode( node ).into_maybe_owned(),
    after.into_maybe_owned() ].concat()
}


fn expressionOutput( node: &Node ) -> String {
  let children = node_children!( node );
  if children.len() > 1 {
    wrapChildrenOutput( "or!( ", node , " )" )
  } else {
    codeForNodeContents( node )
  }
}


fn sequenceOutput( node: &Node ) -> String {
  let children = node_children!( node );
  if children.len() > 1 {
    let mut output = String::from_str( "seq!( " );
    for i in range( 0, children.len() ) {
      output.push_str( codeForNode( children.get( i ) ).as_slice() );
      if i != children.len() -1 {
        output.push_str( ", " );
      }
    }
    output.push_str( " )" );
    output.into_string()
  } else {
    codeForNodeContents( node )
  }
}


fn suffixOutput( node: &Node ) -> String {
  let children = node_children!( node );
  if children.len() == 2 {
    let macro_name = match children.get( 1 ).name {
      "QUESTION" => "opt",
      "STAR" => "star",
      "PLUS" => "plus",
      _ => fail!( "Bad second child." )
    };

    [ macro_name,
      "!( ",
      codeForNode( children.get( 0 ) ).as_slice(),
      " )" ].concat()
  } else {
    codeForNodeContents( node )
  }
}


fn prefixOutput( node: &Node ) -> String {
  let children = node_children!( node );
  if children.len() == 2 {
    let macro_name = children.get( 0 ).name.chars()
      .map( |x| x.to_lowercase() ).collect::<String>();

    [ macro_name.as_slice(),
      "!( ",
      codeForNode( children.get( 1 ) ).as_slice(),
      " )" ].concat()
  } else {
    codeForNodeContents( node )
  }
}


fn primaryOutput( node: &Node ) -> String {
  let children = node_children!( node );
  if children.len() == 1 && children.get( 0 ).name == "Identifier" {
    wrapNodeOutput( "ex!( ", children.get( 0 ), " )" )
  } else {
    codeForNodeContents( node )
  }
}


fn literalOutput( node: &Node ) -> String {
  stringBasedRule( node, "lit" )
}


fn classOutput( node: &Node ) -> String {
  stringBasedRule( node, "class" )
    .replace( r"\\]", r"]" )
    .replace( r"\\[", r"[" )
}


fn escapeToRustLiteral( input: &str ) -> String {
  input.to_string()
    .replace( r"\", r"\\" )
    .replace( "\n", r"\n" )
    .replace( "\t", r"\t" )
    .replace( "\r", r"\r" )
    .replace( "\"", r#"\""# )
}


fn stringBasedRule( node: &Node, rule_name: &str ) -> String {
  let full = codeForNodeContents( node );
  let content = escapeToRustLiteral(
    unescapeString(
      full.as_slice().slice_chars( 1, full.len() - 1 ) ).as_slice() );
  format!( "{}!( \"{}\" )", rule_name, content )
}
