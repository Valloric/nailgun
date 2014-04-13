
use std::str;
use parser::{Node, Children, Data};
use self::unescape::unescapeString;

#[macro_escape]
mod macros;
mod unescape;

// TODO: figure out how to write this as a function on Node; the borrow checker
// was extra painful the last time you tried.
macro_rules! node_children( ( $node:expr ) => ( {
  match $node.contents {
    Children( ref nodes ) => nodes,
    _ => fail!( "No children in node." )
  } } ) )


pub fn codeForNode( node: &Node ) -> ~str {
  match node.name {
    "Definition" => wrapChildrenOutput( ~"rule!( ", node, ~" )\n" ),
    "Expression" => expressionOutput( node ),
    "Sequence" => sequenceOutput( node ),
    "Literal" => literalOutput( node ),
    "Class" => classOutput( node ),
    "Suffix" => suffixOutput( node ),
    "Prefix" => prefixOutput( node ),
    "Primary" => primaryOutput( node ),
    "DOT" => ~"base::Dot",
    "LEFTARROW" => ~" <- ",
    "SLASH" => ~", ",
    "Spacing" => ~"",
    "EndOfLine" => ~"",
    "OPEN" => ~"",
    "CLOSE" => ~"",
    _ => codeForNodeContents( node )
  }
}


fn codeForNodeContents( node: &Node ) -> ~str {
  match node.contents {
    Children( ref children ) => {
      // TODO: use StrBuf here somehow when it's merged in
      children.iter().map( codeForNode ).collect::<Vec<~str>>().concat()
    }
    Data( data ) => str::from_utf8( data ).unwrap().to_owned(),
  }
}


fn wrapChildrenOutput( before: ~str, node: &Node, after: ~str ) -> ~str {
  [ before, codeForNodeContents( node ), after ].concat()
}


fn wrapNodeOutput( before: ~str, node: &Node, after: ~str ) -> ~str {
  [ before, codeForNode( node ), after ].concat()
}


fn expressionOutput( node: &Node ) -> ~str {
  let children = node_children!( node );
  if children.len() > 1 {
    wrapChildrenOutput( ~"or!( ", node , ~" )" )
  } else {
    codeForNodeContents( node )
  }
}


fn sequenceOutput( node: &Node ) -> ~str {
  let children = node_children!( node );
  if children.len() > 1 {
    let mut output = StrBuf::from_str( "seq!( " );
    for i in range( 0, children.len() ) {
      output.push_str( codeForNode( children.get( i ) ) );
      if i != children.len() -1 {
        output.push_str( ", " );
      }
    }
    output.push_str( " )" );
    output.into_owned()
  } else {
    codeForNodeContents( node )
  }
}


fn suffixOutput( node: &Node ) -> ~str {
  let children = node_children!( node );
  if children.len() == 2 {
    let macro_name = match children.get( 1 ).name {
      "QUESTION" => ~"opt",
      "STAR" => ~"star",
      "PLUS" => ~"plus",
      _ => fail!( "Bad second child." )
    };

    [ macro_name, ~"!( ", codeForNode( children.get( 0 ) ), ~" )" ].concat()
  } else {
    codeForNodeContents( node )
  }
}


fn prefixOutput( node: &Node ) -> ~str {
  let children = node_children!( node );
  if children.len() == 2 {
    let macro_name = children.get( 0 ).name.chars()
      .map( |x| x.to_lowercase() ).collect();
    [ macro_name, ~"!( ", codeForNode( children.get( 1 ) ), ~" )" ].concat()
  } else {
    codeForNodeContents( node )
  }
}


fn primaryOutput( node: &Node ) -> ~str {
  let children = node_children!( node );
  if children.len() == 1 && children.get( 0 ).name == "Identifier" {
    wrapNodeOutput( ~"ex!( ", children.get( 0 ), ~" )" )
  } else {
    codeForNodeContents( node )
  }
}


fn literalOutput( node: &Node ) -> ~str {
  stringBasedRule( node, "lit" )
}


fn classOutput( node: &Node ) -> ~str {
  stringBasedRule( node, "class" )
    .replace( r"\]", r"]" )
    .replace( r"\[", r"[" )
}


fn stringBasedRule( node: &Node, rule_name: &str ) -> ~str {
  let content = unescapeString( codeForNodeContents( node ).trim() );

  [ rule_name.to_owned(),
    ~"!( r\"",
    content.slice_chars( 1, content.len() - 1 ).replace( r#"""#, r#"\""# ),
    ~"\" )" ].concat()
}
