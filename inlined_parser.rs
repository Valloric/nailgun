#![allow(dead_code)]

#![feature(slicing_syntax)]
#![allow(non_snake_case)]
#![allow(unstable)]
#![deny(deprecated)]

#[cfg(not(test))]
pub use base::{Node, ParseState, Data, Children, NodeContents, PreOrderNodes};

#[macro_use]
mod base {
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
  mod node {
    use std::fmt;
    use std::str;
    use std::fmt::{Result};
    pub use self::NodeContents::{Data, Children};

    static NO_NAME : &'static str = "<none>";

    pub struct PreOrderNodes<'a, 'b:'a> {
      queue: Vec<&'a Node<'b>>
    }

    impl<'a, 'b:'a> Iterator for PreOrderNodes<'a, 'b> {
      type Item = &'a Node<'b>;

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


    #[derive(Show, PartialEq)]
    pub enum NodeContents<'a> {
      /// A `&[u8]` byte slice this node matched in the parse input. Only leaf nodes
      /// have `Data` contents.
      Data( &'a [u8] ),

      /// Children of the node, if any. Only non-leaf nodes have `Children`
      /// contents.
      Children( Vec<Node<'a>> )
    }


    #[derive(PartialEq)]
    pub struct Node<'a> {
      /// The name of the node.
      pub name: &'static str,

      /// The (inclusive) start index of the range this node matches. It's the byte
      /// (NOT char) offset of the parse input.
      pub start: usize,

      /// The (exclusive) end index of the range this node matches. It's the byte
      /// (NOT char) offset of the parse input.
      pub end: usize,

      /// The contents of the node; this can be either children nodes or a matched
      /// `&[u8]` slice.
      pub contents: NodeContents<'a>
    }


    fn indent( formatter: &mut fmt::Formatter, indent_spaces: u32 )
        -> fmt::Result {
      for _ in range( 0, indent_spaces ) {
        try!( write!( formatter, " " ) )
      }
      Ok(())
    }


    impl<'a> Node<'a> {
      fn format( &self, formatter: &mut fmt::Formatter, indent_spaces: u32 )
          -> fmt::Result {
        try!( indent( formatter, indent_spaces ) );
        try!( write!( formatter,
                      "{0:?} [{1:?}, {2:?}>",
                      self.displayName(), self.start, self.end ) );

        match self.contents {
          Data( data ) => {
            match str::from_utf8( data ) {
              Ok( string ) => {
                try!( writeln!( formatter,
                                ": \"{0:?}\"",
                                string ) );
              }
              _ => {
                try!( writeln!( formatter,
                                ": \"{0:?}\"",
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
      pub fn withoutName( start: usize, end: usize, contents: NodeContents<'a> )
          -> Node<'a> {
        Node { name: "", start: start, end: end, contents: contents }
      }

      /// Creates a `Node` with the provided `name` and makes it a parent of the
      /// provided `children`.
      pub fn withChildren( name: &'static str, mut children: Vec<Node<'a>> )
          -> Node<'a> {
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
              out.push_all( &child.matchedData()[] );
            }
            out
          }
        }
      }
    }

    impl<'a> fmt::Show for Node<'a> {
      fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
        self.format( formatter, 0 )
      }
    }
  }
  #[cfg(test)]
  #[macro_use]
  pub mod test_utils {
    use base::ParseState;

    pub fn ToParseState<'a>( bytes: &'a [u8] ) -> ParseState<'a> {
      ParseState { input: bytes, offset: 0 }
    }

    macro_rules! input_state( ( $ex:expr ) => ( {
          use base::ParseState;
          use std::str::StrExt;
          ParseState { input: $ex.as_bytes(), offset: 0 }
        } ) );
  }

  #[macro_use]
  mod literal {
    use super::{Expression, ParseState, ParseResult};

    macro_rules! lit( ( $ex:expr ) => ( {
          use base;
          use std::str::StrExt;
          &base::Literal::new( $ex.as_bytes() ) } ) );


    pub struct Literal {
      text: &'static [u8]
    }


    impl Literal {
      pub fn new( text: &'static [u8] ) -> Literal {
        Literal { text: text }
      }
    }


    impl Expression for Literal {
      fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
          Option< ParseResult<'a> > {
        if parse_state.input.len() < self.text.len() ||
           parse_state.input.slice_to( self.text.len() ) != self.text {
          return None;
        }

        parse_state.offsetToResult( parse_state.offset + self.text.len() )
      }
    }
  }
  #[macro_use]
  mod char_class {
    use base::unicode::{bytesFollowing, readCodepoint};
    use super::{Expression, ParseState, ParseResult};

    macro_rules! class( ( $ex:expr ) => ( {
          use base;
          use std::str::StrExt;
          &base::CharClass::new( $ex.as_bytes() ) } ) );


    fn toU32Vector( input: &[u8] ) -> Vec<u32> {
      let mut i = 0;
      let mut out_vec : Vec<u32> = vec!();
      loop {
        match input.get( i ) {
          Some( byte ) => match bytesFollowing( *byte ) {
            Some( num_following ) => {
              if num_following > 0 {
                match readCodepoint( input.slice_from( i ) ) {
                  Some( ch ) => {
                    out_vec.push( ch as u32 );
                    i += num_following + 1
                  }
                  _ => { out_vec.push( *byte as u32 ); i += 1 }
                };
              } else { out_vec.push( *byte as u32 ); i += 1 }
            }
            _ => { out_vec.push( *byte as u32 ); i += 1 }
          },
          _ => return out_vec
        }
      }
    }


    pub struct CharClass {
      single_chars: Vec<u32>,
      ranges: Vec<( u32, u32 )>
    }


    impl CharClass {
      pub fn new( contents: &[u8] ) -> CharClass {
        fn rangeAtIndex( index: usize, chars: &[u32] ) -> Option<( u32, u32 )> {
          match ( chars.get( index ),
                  chars.get( index + 1 ),
                  chars.get( index + 2 ) ) {
            ( Some( char1 ), Some( char2 ), Some( char3 ) )
                if *char2 == '-' as u32 => Some( ( *char1, *char3 ) ),
            _ => None
          }
        }

        let chars = toU32Vector( &contents[] );
        let mut char_class = CharClass { single_chars: Vec::new(),
                                         ranges: Vec::new() };
        let mut index = 0;
        loop {
          match rangeAtIndex( index, &chars[] ) {
            Some( range ) => {
              char_class.ranges.push( range );
              index += 3;
            }
            _ => {
              if index >= chars.len() {
                break
              }
              char_class.single_chars.push( chars[ index ] );
              index += 1;
            }
          };
        }

        char_class
      }

      fn matches( &self, character: u32 ) -> bool {
        return self.single_chars.contains( &character ) ||
          self.ranges.iter().any(
            | &(from, to) | character >= from && character <= to );
      }


      fn applyToUtf8<'a>( &self, parse_state: &ParseState<'a> ) ->
          Option< ParseResult<'a> > {
        match readCodepoint( parse_state.input ) {
          Some( ch ) if self.matches( ch as u32 ) => {
            let num_following = bytesFollowing( parse_state.input[ 0 ] ).unwrap();
            parse_state.offsetToResult( parse_state.offset + num_following + 1 )
          }
          _ => None
        }
      }


      fn applyToBytes<'a>( &self, parse_state: &ParseState<'a> ) ->
          Option< ParseResult<'a> > {
        match parse_state.input.get( 0 ) {
          Some( byte ) if self.matches( *byte as u32 ) => {
            parse_state.offsetToResult( parse_state.offset + 1 )
          }
          _ => None
        }
      }
    }


    impl Expression for CharClass {
      fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
          Option< ParseResult<'a> > {
        match self.applyToUtf8( parse_state ) {
          Some( x ) => Some( x ),
          _ => self.applyToBytes( parse_state )
        }
      }
    }
  }
  #[macro_use]
  mod not {
    use super::{Expression, ParseState, ParseResult};

    macro_rules! not( ( $ex:expr ) => ( {
        use base;
        &base::NotEx::new($ex) } ); );

    pub struct NotEx<'a> {
      expr: &'a ( Expression + 'a )
    }


    impl<'a> NotEx<'a> {
      pub fn new( expr: &Expression ) -> NotEx {
        NotEx { expr: expr }
      }
    }


    impl<'b> Expression for NotEx<'b> {
      fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
          Option< ParseResult<'a> > {
        match self.expr.apply( parse_state ) {
          Some( _ ) => None,
          _ => Some( ParseResult::fromParseState( *parse_state ) )
        }
      }
    }
  }
  #[macro_use]
  mod and {

    use super::{Expression, ParseState, ParseResult};

    macro_rules! and( ( $ex:expr ) => ( {
        use base;
        &base::And::new( $ex ) } ); );

    pub struct And<'a> {
      expr: &'a ( Expression + 'a )
    }


    impl<'a> And<'a> {
      pub fn new( expr: &Expression ) -> And {
        And { expr: expr }
      }
    }


    impl<'b> Expression for And<'b> {
      fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
          Option< ParseResult<'a> > {
        match self.expr.apply( parse_state ) {
          Some( _ ) => Some( ParseResult::fromParseState( *parse_state ) ),
          _ => None
        }
      }
    }
  }
  mod dot {
    use super::{Expression, ParseState, ParseResult};
    use base::unicode::{bytesFollowing, readCodepoint};

    pub struct Dot;
    impl Expression for Dot {
      fn apply<'a>( &self, parse_state: &ParseState<'a> ) -> Option< ParseResult<'a> > {
        match readCodepoint( parse_state.input ) {
          Some( _ ) => {
            let num_following = bytesFollowing( parse_state.input[ 0 ] ).unwrap();
            return parse_state.offsetToResult(
              parse_state.offset + num_following + 1 )
          }
          _ => ()
        }

        match parse_state.input.get( 0 ) {
          Some( _ ) => parse_state.offsetToResult( parse_state.offset + 1 ),
          _ => None
        }
      }
    }
  }
  #[macro_use]
  mod option {
    use super::{Expression, ParseState, ParseResult};

    macro_rules! opt( ( $ex:expr ) => ( {
        use base;
        &base::OptionEx::new( $ex ) } ); );

    pub struct OptionEx<'a> {
      expr: &'a ( Expression + 'a )
    }


    impl<'a> OptionEx<'a> {
      pub fn new( expr: &Expression ) -> OptionEx {
        OptionEx { expr: expr }
      }
    }


    impl<'b> Expression for OptionEx<'b> {
      fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
          Option< ParseResult<'a> > {
        match self.expr.apply( parse_state ) {
          result @ Some( _ ) => result,
          _ => Some( ParseResult::fromParseState( *parse_state ) )
        }
      }
    }
  }
  #[macro_use]
  mod star {
    use super::{Expression, ParseState, ParseResult};

    macro_rules! star( ( $ex:expr ) => ( {
        use base;
        &base::Star::new( $ex ) } ); );

    pub struct Star<'a> {
      expr: &'a ( Expression + 'a )
    }


    impl<'b> Star<'b> {
      pub fn new( expr: &Expression ) -> Star {
        Star { expr: expr }
      }
    }


    impl<'b> Expression for Star<'b> {
      fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
          Option< ParseResult<'a> > {
        let mut final_result = ParseResult::fromParseState( *parse_state );
        loop {
          match self.expr.apply( &final_result.parse_state ) {
            Some( result ) => {
              final_result.parse_state = result.parse_state;
              final_result.nodes.extend( result.nodes.into_iter() );
            }
            _ => break
          }
        }
        Some( final_result )
      }
    }
  }
  #[macro_use]
  mod plus {
    use super::{Expression, ParseState, ParseResult};

    macro_rules! plus( ( $ex:expr ) => ( {
        use base;
        &base::Plus::new( $ex ) } ); );

    pub struct Plus<'a> {
      expr: &'a ( Expression + 'a )
    }


    impl<'b> Plus<'b> {
      pub fn new( expr: &Expression ) -> Plus {
        Plus { expr: expr }
      }
    }


    impl<'b> Expression for Plus<'b> {
      fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
          Option< ParseResult<'a> > {
        let mut final_result = ParseResult::fromParseState( *parse_state );
        let mut num_matches = 0;
        loop {
          match self.expr.apply( &final_result.parse_state ) {
            Some( result ) => {
              final_result.parse_state = result.parse_state;
              final_result.nodes.extend( result.nodes.into_iter() );
              num_matches += 1;
            }
            _ => break
          }
        }

        if num_matches > 0 {
          Some( final_result )
        } else {
          None
        }
      }
    }
  }
  #[macro_use]
  mod or {
    use super::{Expression, ParseState, ParseResult};

    macro_rules! or( ( $( $ex:expr ),* ) => ( {
        use base;
        &base::Or::new( &[ $( $ex ),* ] ) } ); );

    pub struct Or<'a> {
      exprs: &'a [&'a (Expression + 'a)]
    }


    impl<'b> Or<'b> {
      pub fn new<'a>( exprs: &'a [&Expression] ) -> Or<'a> {
        Or { exprs: exprs }
      }
    }


    impl<'b> Expression for Or<'b> {
      fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
          Option< ParseResult<'a> > {
        for expr in self.exprs.iter() {
          match expr.apply( parse_state ) {
            result @ Some( _ ) => return result,
            _ => ()
          }
        }
        None
      }
    }
  }
  #[macro_use]
  mod fuse {
    use super::{Expression, ParseState, ParseResult};

    macro_rules! fuse( ( $ex:expr ) => ( {
        use base;
        &base::Fuse::new( $ex ) } ); );

    pub struct Fuse<'a> {
      expr: &'a ( Expression + 'a )
    }


    impl<'a> Fuse<'a> {
      pub fn new( expr: & Expression ) -> Fuse {
        Fuse { expr: expr }
      }
    }


    impl<'b> Expression for Fuse<'b> {
      fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
          Option< ParseResult<'a> > {
        match self.expr.apply( parse_state ) {
          Some( result ) => {
            parse_state.offsetToResult( result.parse_state.offset )
          },
          _ => None
        }
      }
    }
  }
  #[macro_use]
  mod sequence {
    use super::{Expression, ParseState, ParseResult};

    macro_rules! seq( ( $( $ex:expr ),* ) => ( {
        use base;
        &base::Sequence::new( &[ $( $ex ),* ] ) } ); );

    pub struct Sequence<'a> {
      exprs: &'a [&'a (Expression + 'a)]
    }


    impl<'b> Sequence<'b> {
      pub fn new<'a>( exprs: &'a [&Expression] ) -> Sequence<'a> {
        Sequence { exprs: exprs }
      }
    }


    impl<'b> Expression for Sequence<'b> {
      fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
          Option< ParseResult<'a> > {
        let mut final_result = ParseResult::fromParseState( *parse_state );
        for expr in self.exprs.iter() {
          match expr.apply( &final_result.parse_state ) {
            Some( result ) => {
              final_result.parse_state = result.parse_state;
              final_result.nodes.extend( result.nodes.into_iter() );
            }
            _ => return None
          }
        }
        Some( final_result )
      }
    }
  }
  #[macro_use]
  mod wrap {
    use super::{Expression, ParseState, ParseResult, Rule};

    macro_rules! ex( ( $ex:expr ) => ( {
        use base;
        &base::WrapEx{ rule: $ex } } ); );

    pub struct WrapEx {
      pub rule: Rule
    }


    impl Expression for WrapEx {
      fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
          Option< ParseResult<'a> > {
        (self.rule)( parse_state )
      }
    }
  }
  mod unicode {
    use std::char;
    pub static UTF8_1BYTE_FOLLOWING: u8 = 0b11000000;
    pub static UTF8_2BYTE_FOLLOWING: u8 = 0b11100000;
    pub static UTF8_3BYTE_FOLLOWING: u8 = 0b11110000;

    pub fn readCodepoint( input: &[u8] ) -> Option< char > {
      fn isContinuationByte( byte: u8 ) -> bool {
        byte & 0b11000000 == 0b10000000
      }

      fn codepointBitsFromLeadingByte( byte: u8 ) -> u32 {
        let good_bits =
          if isAscii( byte ) {
            byte
          } else if byte & 0b11100000 == UTF8_1BYTE_FOLLOWING {
            byte & 0b00011111
          } else if byte & 0b11110000 == UTF8_2BYTE_FOLLOWING {
            byte & 0b00001111
          } else {
            byte & 0b00000111
          };
        good_bits as u32
      }

      fn codepointBitsFromContinuationByte( byte: u8 ) -> u32 {
        ( byte & 0b00111111 ) as u32
      }

      match input.get( 0 ) {
        Some( first_byte ) => {
          match bytesFollowing( *first_byte ) {
            Some( num_following ) => {
              let mut codepoint: u32 =
                codepointBitsFromLeadingByte( *first_byte ) << 6 * num_following;
              for i in range( 1, num_following + 1 ) {
                match input.get( i ) {
                  Some( byte ) if isContinuationByte( *byte ) => {
                    codepoint |= codepointBitsFromContinuationByte( *byte ) <<
                      6 * ( num_following - i );
                  }
                  _ => return None
                }
              }
              char::from_u32( codepoint )
            }
            _ => None
          }
        }
        _ => None
      }
    }


    pub fn bytesFollowing( byte: u8 ) -> Option< usize > {
      if isAscii( byte ) {
        Some( 0 )
      } else if byte & 0b11100000 == UTF8_1BYTE_FOLLOWING {
        Some( 1 )
      } else if byte & 0b11110000 == UTF8_2BYTE_FOLLOWING {
        Some( 2 )
      } else if byte & 0b11111000 == UTF8_3BYTE_FOLLOWING {
        Some( 3 )
      } else {
        None
      }
    }


    pub fn isAscii( byte: u8 ) -> bool {
      return byte & 0b10000000 == 0;
    }
  }


  #[doc(hidden)]
  #[derive(Show, Clone, PartialEq, Copy)]
  pub struct ParseState<'a> {
    pub input: &'a [u8],
    pub offset: usize
  }


  impl<'a> ParseState<'a> {
    fn advanceTo( &self, new_offset: usize ) -> ParseState<'a> {
      let mut clone = self.clone();
      clone.input = clone.input.slice_from( new_offset - clone.offset );
      clone.offset = new_offset;
      clone
    }

    fn sliceTo( &self, new_offset: usize ) -> &'a [u8] {
      self.input.slice_to( new_offset - self.offset )
    }

    fn offsetToResult( &self, new_offset: usize )
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
    pub fn oneNode( node: Node<'a>, parse_state: ParseState<'a> )
        -> ParseResult<'a> {
      ParseResult { nodes: vec!( node ), parse_state: parse_state }
    }

    pub fn fromParseState( parse_state: ParseState<'a> ) -> ParseResult<'a> {
      ParseResult { nodes: vec!(), parse_state: parse_state }
    }
  }


  pub trait Expression {
    fn apply<'a>( &self, parse_state: &ParseState<'a> )
        -> Option< ParseResult<'a> >;
  }

  type Rule = for<'a> fn( &ParseState<'a> ) -> Option< ParseResult<'a> >;
}

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
      use std::option::Option::{Some, None};

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
);

#[cfg(not(test))]
pub fn parse<'a>( input: &'a [u8] ) -> Option< Node<'a> > {
  let parse_state = ParseState { input: input, offset: 0 };
  match rules::Grammar( &parse_state ) {
    Some( result ) => Some( result.nodes.into_iter().next().unwrap() ),
    _ => None
  }
}


mod rules {
  #![no_implicit_prelude]

  use base;
  use std;

  rule!( Grammar <- seq!( ex!( Spacing ), plus!( ex!( Definition ) ), ex!( EndOfFile ) ) );
  rule!( Definition <- seq!( ex!( Identifier ), ex!( ARROW ), ex!( Expression ) ) );
  rule!( Expression <- seq!( ex!( Sequence ), star!( seq!( ex!( SLASH ), ex!( Sequence ) ) ) ) );
  rule!( Sequence <- star!( ex!( Prefix ) ) );
  rule!( Prefix <- seq!( opt!( or!( ex!( AND ), ex!( NOT ), ex!( FUSE ) ) ), ex!( Suffix ) ) );
  rule!( Suffix <- seq!( ex!( Primary ), opt!( or!( ex!( QUESTION ), ex!( STAR ), ex!( PLUS ) ) ) ) );
  rule!( Primary <- or!( seq!( ex!( Identifier ), not!( ex!( ARROW ) ) ), seq!( ex!( OPEN ), ex!( Expression ), ex!( CLOSE ) ), ex!( Literal ), ex!( Class ), ex!( DOT ) ) );
  rule!( Identifier <- seq!( fuse!( seq!( ex!( IdentStart ), star!( ex!( IdentCont ) ) ) ), ex!( Spacing ) ) );
  rule!( IdentStart <- class!( "a-zA-Z_" ) );
  rule!( IdentCont <- or!( ex!( IdentStart ), class!( "0-9" ) ) );
  rule!( Literal <- seq!( fuse!( or!( seq!( class!( "'" ), star!( seq!( not!( class!( "'" ) ), ex!( Char ) ) ), class!( "'" ) ), seq!( class!( "\"" ), star!( seq!( not!( class!( "\"" ) ), ex!( Char ) ) ), class!( "\"" ) ) ) ), ex!( Spacing ) ) );
  rule!( Class <- seq!( lit!( "[" ), star!( seq!( not!( lit!( "]" ) ), ex!( Range ) ) ), lit!( "]" ), ex!( Spacing ) ) );
  rule!( Range <- or!( seq!( ex!( Char ), lit!( "-" ), ex!( Char ) ), ex!( Char ) ) );
  rule!( Char <- or!( seq!( lit!( "\\" ), class!( "nrt'\"[]\\" ) ), seq!( lit!( "\\" ), class!( "0-2" ), class!( "0-7" ), class!( "0-7" ) ), seq!( lit!( "\\" ), class!( "0-7" ), opt!( class!( "0-7" ) ) ), seq!( not!( lit!( "\\" ) ), &base::Dot ) ) );
  rule!( ARROW <- or!( ex!( FUSEARROW ), ex!( LEFTARROW ) ) );
  rule!( LEFTARROW <- seq!( lit!( "<-" ), ex!( Spacing ) ) );
  rule!( FUSEARROW <- seq!( lit!( "<~" ), ex!( Spacing ) ) );
  rule!( SLASH <- seq!( lit!( "/" ), ex!( Spacing ) ) );
  rule!( AND <- seq!( lit!( "&" ), ex!( Spacing ) ) );
  rule!( NOT <- seq!( lit!( "!" ), ex!( Spacing ) ) );
  rule!( QUESTION <- seq!( lit!( "?" ), ex!( Spacing ) ) );
  rule!( STAR <- seq!( lit!( "*" ), ex!( Spacing ) ) );
  rule!( PLUS <- seq!( lit!( "+" ), ex!( Spacing ) ) );
  rule!( OPEN <- seq!( lit!( "(" ), ex!( Spacing ) ) );
  rule!( CLOSE <- seq!( lit!( ")" ), ex!( Spacing ) ) );
  rule!( DOT <- seq!( lit!( "." ), ex!( Spacing ) ) );
  rule!( FUSE <- seq!( lit!( "~" ), ex!( Spacing ) ) );
  rule!( Spacing <- fuse!( star!( or!( ex!( Space ), ex!( Comment ) ) ) ) );
  rule!( Comment <- fuse!( seq!( lit!( "#" ), star!( seq!( not!( ex!( EndOfLine ) ), &base::Dot ) ), ex!( EndOfLine ) ) ) );
  rule!( Space <- or!( lit!( " " ), lit!( "\t" ), ex!( EndOfLine ) ) );
  rule!( EndOfLine <- or!( lit!( "\r\n" ), lit!( "\n" ), lit!( "\r" ) ) );
  rule!( EndOfFile <- not!( &base::Dot ) );
  
}
