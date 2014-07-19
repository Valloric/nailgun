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
#![feature(macro_rules)]
#![allow(non_snake_case_functions)]

#[cfg(not(test))]
pub use base::{Node, ParseState, Data, Children, NodeContents, PreOrderNodes};

#[macro_escape]
mod base;

macro_rules! rule(
  (
    $name:ident <- $body:expr
  ) => (
    pub fn $name<'a>( parse_state: &base::ParseState<'a> )
         -> std::option::Option< base::ParseResult<'a> > {
      use base::Expression;
      use base::Node;
      use base::ParseResult;
      use std::clone::Clone;
      use std::option::{Some, None};

      match $body.apply( parse_state ) {
        Some( result ) => {
          let state = result.parse_state.clone();
          Some( ParseResult::oneNode(
              Node::withChildren( stringify!( $name ), result.nodes ), state ) )
        }
        _ => None
      }
    }
  );
)

#[cfg(not(test))]
pub fn parse<'a>( input: &'a [u8] ) -> Option< Node<'a> > {
  let parse_state = ParseState { input: input, offset: 0 };
  match rules::Grammar( &parse_state ) {
    Some( result ) => Some( result.nodes.move_iter().next().unwrap() ),
    _ => None
  }
}


mod rules {
  #![no_implicit_prelude]

  use base;
  use std;

  // RULES START

  rule!( Grammar <- seq!( ex!( Spacing ), plus!( ex!( Definition ) ), ex!( EndOfFile ) ) )
  rule!( Definition <- seq!( ex!( Identifier ), ex!( ARROW ), ex!( Expression ) ) )
  rule!( Expression <- seq!( ex!( Sequence ), star!( seq!( ex!( SLASH ), ex!( Sequence ) ) ) ) )
  rule!( Sequence <- star!( ex!( Prefix ) ) )
  rule!( Prefix <- seq!( opt!( or!( ex!( AND ), ex!( NOT ), ex!( FUSE ) ) ), ex!( Suffix ) ) )
  rule!( Suffix <- seq!( ex!( Primary ), opt!( or!( ex!( QUESTION ), ex!( STAR ), ex!( PLUS ) ) ) ) )
  rule!( Primary <- or!( seq!( ex!( Identifier ), not!( ex!( ARROW ) ) ), seq!( ex!( OPEN ), ex!( Expression ), ex!( CLOSE ) ), ex!( Literal ), ex!( Class ), ex!( DOT ) ) )
  rule!( Identifier <- seq!( fuse!( seq!( ex!( IdentStart ), star!( ex!( IdentCont ) ) ) ), ex!( Spacing ) ) )
  rule!( IdentStart <- class!( "a-zA-Z_" ) )
  rule!( IdentCont <- or!( ex!( IdentStart ), class!( "0-9" ) ) )
  rule!( Literal <- seq!( fuse!( or!( seq!( class!( "'" ), star!( seq!( not!( class!( "'" ) ), ex!( Char ) ) ), class!( "'" ) ), seq!( class!( "\"" ), star!( seq!( not!( class!( "\"" ) ), ex!( Char ) ) ), class!( "\"" ) ) ) ), ex!( Spacing ) ) )
  rule!( Class <- seq!( lit!( "[" ), star!( seq!( not!( lit!( "]" ) ), ex!( Range ) ) ), lit!( "]" ), ex!( Spacing ) ) )
  rule!( Range <- or!( seq!( ex!( Char ), lit!( "-" ), ex!( Char ) ), ex!( Char ) ) )
  rule!( Char <- or!( seq!( lit!( "\\" ), class!( "nrt'\"[]\\" ) ), seq!( lit!( "\\" ), class!( "0-2" ), class!( "0-7" ), class!( "0-7" ) ), seq!( lit!( "\\" ), class!( "0-7" ), opt!( class!( "0-7" ) ) ), seq!( not!( lit!( "\\" ) ), base::Dot ) ) )
  rule!( ARROW <- or!( ex!( FUSEARROW ), ex!( LEFTARROW ) ) )
  rule!( LEFTARROW <- seq!( lit!( "<-" ), ex!( Spacing ) ) )
  rule!( FUSEARROW <- seq!( lit!( "<~" ), ex!( Spacing ) ) )
  rule!( SLASH <- seq!( lit!( "/" ), ex!( Spacing ) ) )
  rule!( AND <- seq!( lit!( "&" ), ex!( Spacing ) ) )
  rule!( NOT <- seq!( lit!( "!" ), ex!( Spacing ) ) )
  rule!( QUESTION <- seq!( lit!( "?" ), ex!( Spacing ) ) )
  rule!( STAR <- seq!( lit!( "*" ), ex!( Spacing ) ) )
  rule!( PLUS <- seq!( lit!( "+" ), ex!( Spacing ) ) )
  rule!( OPEN <- seq!( lit!( "(" ), ex!( Spacing ) ) )
  rule!( CLOSE <- seq!( lit!( ")" ), ex!( Spacing ) ) )
  rule!( DOT <- seq!( lit!( "." ), ex!( Spacing ) ) )
  rule!( FUSE <- seq!( lit!( "~" ), ex!( Spacing ) ) )
  rule!( Spacing <- fuse!( star!( or!( ex!( Space ), ex!( Comment ) ) ) ) )
  rule!( Comment <- fuse!( seq!( lit!( "#" ), star!( seq!( not!( ex!( EndOfLine ) ), base::Dot ) ), ex!( EndOfLine ) ) ) )
  rule!( Space <- or!( lit!( " " ), lit!( "\t" ), ex!( EndOfLine ) ) )
  rule!( EndOfLine <- or!( lit!( "\r\n" ), lit!( "\n" ), lit!( "\r" ) ) )
  rule!( EndOfFile <- not!( base::Dot ) )

  // RULES END

  #[cfg(test)]
  mod tests {
    use super::{EndOfFile, EndOfLine, Space, Comment, Spacing, Char, Range,
                Class, Literal, Identifier, Definition, Grammar};

    macro_rules! consumes(
      (
        $name:ident, $input:expr
      ) => (
        {
          use base::ParseResult;
          use std::option::Some;
          use std::collections::Collection;

          match $name( &input_state!( $input ) ) {
            Some( ParseResult{ nodes: _,
                              parse_state: parse_state } ) => {
              parse_state.input.is_empty()
            }
            _ => false
          }
        }
      );
    )

    macro_rules! matches(
      (
        $name:ident, $input:expr
      ) => (
        $name( &input_state!( $input ) ).is_some()
      );
    )

    #[test]
    fn Grammar_Works() {
      assert!( consumes!( Grammar,
                          r#"
        h16           <- HEXDIGIT (HEXDIGIT (HEXDIGIT HEXDIGIT?)?)?
        ls32          <- h16 ":" h16 / IPv4address
        IPv4address   <- dec_octet "." dec_octet "." dec_octet "." dec_octet"#
        ) )
    }

    #[test]
    fn Definition_Works() {
      assert!( consumes!( Definition,
                          "Grammar <- Spacing Definition+ EndOfFile" ) )
      assert!( consumes!( Definition,
                          r#"Char <- '\\' [nrt'"\[\]\\] /
                                    '\\' [0-2][0-7][0-7] /
                                    '\\' [0-7][0-7]? /
                                    !'\\' ."# ) )
    }

    #[test]
    fn Identifier_Works() {
      assert!( consumes!( Identifier, "abc" ) );
      assert!( consumes!( Identifier, "a" ) );
      assert!( consumes!( Identifier, "_" ) );
      assert!( consumes!( Identifier, "a123" ) );
      assert!( consumes!( Identifier, "a  \n" ) );

      assert!( !consumes!( Identifier, "1a" ) );
      assert!( !consumes!( Identifier, "1" ) );
      assert!( !consumes!( Identifier, "Ć" ) );
    }

    #[test]
    fn Literal_Works() {
      assert!( consumes!( Literal, "'abc'" ) );
      assert!( consumes!( Literal, r#""abc""# ) );
      assert!( consumes!( Literal, "'abc'  \n" ) );
      assert!( !consumes!( Literal, "'abc''bb'" ) );
    }

    #[test]
    fn Class_Works() {
      assert!( consumes!( Class, "[a-z]" ) );
      assert!( consumes!( Class, "[a-z]  \n" ) );
      assert!( consumes!( Class, "[abc]" ) );
      assert!( consumes!( Class, "[abc0-9g]" ) );
    }

    #[test]
    fn Range_Works() {
      assert!( consumes!( Range, "a-z" ) );
      assert!( consumes!( Range, "a" ) );
    }

    #[test]
    fn Char_Works() {
      assert!( consumes!( Char, r"\n" ) );
      assert!( consumes!( Char, r"\]" ) );
      assert!( consumes!( Char, r"\\" ) );
      assert!( consumes!( Char, r"\'" ) );
      assert!( consumes!( Char, "a" ) );
      assert!( consumes!( Char, "x" ) );
      assert!( consumes!( Char, "Ć" ) );
      assert!( consumes!( Char, "€" ) );
      assert!( consumes!( Char, r"\277" ) );
      assert!( consumes!( Char, r"\77" ) );
      assert!( consumes!( Char, r"\7" ) );

      assert!( !consumes!( Char, "aa" ) );
    }

    #[test]
    fn Spacing_Works() {
      assert!( consumes!( Spacing, "  \t #g\n" ) );
      assert!( consumes!( Spacing, "#a\n  #1\n" ) );

      // Spacing DOES match here because at the top level, it is a star
      // expression which can match consuming nothing.
      assert!( matches!( Spacing, "" ) );
      assert!( !consumes!( Spacing, "#" ) );
      assert!( !consumes!( Spacing, "a" ) );
    }

    #[test]
    fn Comment_Works() {
      assert!( consumes!( Comment, "#\n" ) );
      assert!( consumes!( Comment, "# foo! \n" ) );
      assert!( !matches!( Comment, "\n" ) );
      assert!( !matches!( Comment, "#" ) );
      assert!( !matches!( Comment, "a" ) );
    }

    #[test]
    fn Space_Works() {
      assert!( consumes!( Space, " " ) );
      assert!( consumes!( Space, "\t" ) );
      assert!( consumes!( Space, "\n" ) );
      assert!( !matches!( Space, "a" ) );
    }

    #[test]
    fn EndOfLine_Works() {
      assert!( consumes!( EndOfLine, "\n" ) );
      assert!( consumes!( EndOfLine, "\r" ) );
      assert!( consumes!( EndOfLine, "\r\n" ) );
      assert!( !matches!( EndOfLine, "a" ) );
    }

    #[test]
    fn EndOfFile_Works() {
      assert!( consumes!( EndOfFile, "" ) );
      assert!( !matches!( EndOfFile, "a" ) );
    }
  }
}