use std::str::Chars;
use std::str::from_char;
use std::iter::Iterator;
use std::iter::Enumerate;

static LITERAL_EXPRESSION : &'static str = "LiteralExpression";
static DOT_EXPRESSION : &'static str = "DotExpression";
static CHAR_CLASS_EXPRESSION : &'static str = "CharClassExpression";

#[deriving(Eq)]
enum NodeContents {
  Text( ~str ),
  Children( ~[ ~Node ] )
}


#[deriving(Eq)]
struct Node {
  name: &'static str,
  start: uint,
  end: uint,
  contents: NodeContents
}


impl Node {
  fn matchedText( &self ) -> ~str {
    match self.contents {
      Text( ref x ) => x.to_owned(),

      // TODO: recurse through children and collect all text
      Children( _ ) => ~"foo"
    }
  }
}


type ParseState<'a> = Enumerate< Chars<'a> >;

struct ParseResult<'a> {
  node: Option< Node >,
  parse_state: ParseState<'a>
}


trait Expression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) -> Option< ParseResult<'a> >;
}


struct LiteralExpression {
  text: &'static str
}


impl Expression for LiteralExpression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    let indices_and_chars =
      parse_state.take( self.text.len() ).collect::< ~[ ( uint, char ) ] >();
    let chars : ~str =
      indices_and_chars.iter().map( | &( _, ch ) | ch ).collect();

    if self.text == chars {
      Some( ParseResult { parse_state: Increment( parse_state.clone(),
                                                  self.text.len() ),
                          node: Some( Node {
                            name: LITERAL_EXPRESSION,
                            start: indices_and_chars.head().unwrap().val0(),
                            end: indices_and_chars.last().unwrap().val0() + 1,
                            contents: Text( self.text.to_owned() ) } ) } )
    } else {
      None
    }
  }
}


struct DotExpression;
impl Expression for DotExpression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) -> Option< ParseResult<'a> > {
    let mut new_parse_state = parse_state.clone();
    match new_parse_state.next() {
      Some( ( index, character ) ) => Some(
        ParseResult { parse_state: new_parse_state,
                      node: Some( Node {
                        name: DOT_EXPRESSION,
                        start: index,
                        end: index + 1,
                        contents: Text( from_char( character ) ) } ) } ),
      _ => None
    }
  }
}


struct NotExpression;
impl Expression for NotExpression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) -> Option< ParseResult<'a> > {
    None
  }
}


struct CharClassExpression {
  // All the single chars in the char class
  single_chars: ~[char],

  // Sequence of [from, to] (inclusive bounds) char ranges
  ranges: ~[(char, char)]
}


impl CharClassExpression {
  // Takes the inner content of square brackets, so for [a-z], send "a-z".
  fn new( contents: &str ) -> CharClassExpression {
    let mut char_class = CharClassExpression { single_chars: ~[],
                                               ranges: ~[] };
    let mut in_range = false;
    let mut prev_char : Option< char > = None;
    for character in contents.chars() {
      if in_range {
        char_class.ranges.push( ( prev_char.unwrap(), character ) );
        in_range = false;
        prev_char = None;
      } else {
        if character == '-' {
          if prev_char.is_some() {
            in_range = true;
          } else {
            char_class.single_chars.push( '-' );
          }
        } else {
          if prev_char.is_some() {
            char_class.single_chars.push( prev_char.unwrap() );
          }
          prev_char = Some( character );
        }
      }
    }

    if prev_char.is_some() {
      char_class.single_chars.push( prev_char.unwrap() )
    }

    // Handles char classes like [a-]
    if in_range {
      char_class.single_chars.push( '-' );
    }
    char_class
  }

  fn matches( &self, character: char ) -> bool {
    return self.single_chars.contains( &character ) ||
      self.ranges.iter().any(
        | &( from, to ) | character >= from && character <= to );
  }
}


impl Expression for CharClassExpression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    let mut new_parse_state = parse_state.clone();
    match new_parse_state.next() {
      Some( ( index, ch ) ) if self.matches( ch ) => {
        Some( ParseResult { parse_state: new_parse_state,
                      node: Some( Node {
                        name: CHAR_CLASS_EXPRESSION,
                        start: index,
                        end: index + 1,
                        contents: Text( from_char( ch ) ) } ) } )
      }
      _ => None
    }
  }
}


fn Increment<B, T: Iterator<B> >( mut iter: T, steps: uint ) -> T {
  for i in range( 0, steps ) {
    iter.next();
  }
  iter
}

pub fn parseString( input: &str ) -> Node {
  Node { name: LITERAL_EXPRESSION,
         start: 0,
         end: 3,
         contents: Text( input.to_owned() ) }
}


#[cfg(test)]
mod parser_tests;
