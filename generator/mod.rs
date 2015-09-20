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
use std::str;
use std::ascii::AsciiExt;
use inlined_parser::{Node, Children, Data};
use self::unescape::unescapeString;

mod unescape;

// TODO: figure out how to write this as a function on Node; the borrow checker
// was extra painful the last time you tried.
macro_rules! node_children( ( $node:expr ) => ( {
  match $node.contents {
    Children( ref nodes ) => nodes,
    _ => panic!( "No children in node." )
  } } ) );


pub fn codeForNode( node: &Node ) -> String {
  match node.name {
    "Definition" => definitionOutput( node ),
    "Expression" => expressionOutput( node ),
    "Sequence" => sequenceOutput( node ),
    "Literal" => literalOutput( node ),
    "Class" => classOutput( node ),
    "Suffix" => suffixOutput( node ),
    "Prefix" => prefixOutput( node ),
    "Primary" => primaryOutput( node ),
    "DOT" => String::from( "&base::Dot" ),
    "ARROW" => String::from( " <- " ),
    "SLASH" => String::from( ", " ),
    "Spacing" | "EndOfLine" | "OPEN" | "CLOSE" => String::new(),
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
  before.to_string() + &codeForNodeContents( node ) + after
}


fn wrapNodeOutput( before: &str, node: &Node, after: &str ) -> String {
  before.to_string() + &codeForNode( node ) + after
}


fn definitionOutput( node: &Node ) -> String {
  fn arrowName( node: &Node ) -> &'static str {
    match node.contents {
      Children( ref nodes ) => {
        match nodes[ 1 ].contents {
          Children( ref nodes2 ) => {
            nodes2[ 0 ].name
          },
          _ => panic!( "No children in node.")
        }
      },
      _ => panic!( "No children in node.")
    }
  }

  let children = node_children!( node );
  let inner_code = match arrowName( node ) {
    "FUSEARROW" => {
      codeForNode( &children[ 0 ] ) +
      &codeForNode( &children[ 1 ] ) +
      "fuse!( " +
      &codeForNode( &children[ 2 ] ) +
      " )"
    },
    _ => codeForNodeContents( node )
  };

  "rule!( ".to_string() + &inner_code + " );\n"
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
    let mut output = String::from( "seq!( " );
    for i in 0 .. children.len() {
      output.push_str( &codeForNode( &children[ i ] ) );
      if i != children.len() -1 {
        output.push_str( ", " );
      }
    }
    output.push_str( " )" );
    output
  } else {
    codeForNodeContents( node )
  }
}


fn suffixOutput( node: &Node ) -> String {
  let children = node_children!( node );
  if children.len() == 2 {
    let macro_name = match children[ 1 ].name {
      "QUESTION" => "opt",
      "STAR" => "star",
      "PLUS" => "plus",
      _ => panic!( "Bad second child." )
    };

    macro_name.to_string() + "!( " + &codeForNode( &children[ 0 ] ) + " )"
  } else {
    codeForNodeContents( node )
  }
}


fn prefixOutput( node: &Node ) -> String {
  let children = node_children!( node );
  if children.len() == 2 {
    let macro_name = children[ 0 ].name.to_ascii_lowercase();
    macro_name + "!( " + &codeForNode( &children[ 1 ] ) + " )"
  } else {
    codeForNodeContents( node )
  }
}


fn primaryOutput( node: &Node ) -> String {
  let children = node_children!( node );
  if children.len() == 1 && children[ 0 ].name == "Identifier" {
    wrapNodeOutput( "ex!( ", &children[ 0 ], " )" )
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
    &unescapeString( &full[ 1 .. full.len() - 1 ] ) );
  format!( "{}!( \"{}\" )", rule_name, content )
}
