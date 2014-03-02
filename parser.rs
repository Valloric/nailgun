use std::str::Chars;
use std::str::from_char;
use std::iter::Iterator;
use std::iter::Enumerate;

static LITERAL_EXPRESSION : &'static str = "LiteralExpression";
static DOT_EXPRESSION : &'static str = "DotExpression";
static CHAR_CLASS_EXPRESSION : &'static str = "CharClassExpression";
static NOT_EXPRESSION : &'static str = "NotExpression";

#[deriving(Eq)]
enum NodeContents {
  Text( ~str ),
  Children( ~[ ~Node ] ),
  Empty
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
      Empty => ~"",
      Text( ref x ) => x.to_owned(),

      // TODO: recurse through children and collect all text
      Children( _ ) => ~"foo"
    }
  }

  fn predicate( name: &'static str ) -> Node {
    Node { name: name, start: 0, end: 0, contents: Empty }
  }
}


type ParseState<'a> = Enumerate< Chars<'a> >;

struct ParseResult<'a> {
  node: Node,
  parse_state: ParseState<'a>
}


trait Expression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) -> Option< ParseResult<'a> >;
}


struct LiteralExpression {
  text: &'static str
}


impl LiteralExpression {
  fn new( text: &'static str ) -> LiteralExpression {
    LiteralExpression { text: text }
  }
}


impl Expression for LiteralExpression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    let indices_and_chars =
      parse_state.take( self.text.len() ).collect::< ~[ ( uint, char ) ] >();
    let chars : ~str =
      indices_and_chars.iter().map( | &( _, ch ) | ch ).collect();

    if self.text == chars {
      Some( ParseResult { parse_state: MoveForward( parse_state.clone(),
                                                    self.text.len() ),
                          node: Node {
                            name: LITERAL_EXPRESSION,
                            start: indices_and_chars.head().unwrap().val0(),
                            end: indices_and_chars.last().unwrap().val0() + 1,
                            contents: Text( self.text.to_owned() ) } } )
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
                      node: Node {
                        name: DOT_EXPRESSION,
                        start: index,
                        end: index + 1,
                        contents: Text( from_char( character ) ) } } ),
      _ => None
    }
  }
}


struct CharClassExpression {
  // All the single chars in the char class
  single_chars: ~[ char ],

  // Sequence of [from, to] (inclusive bounds) char ranges
  ranges: ~[ ( char, char ) ]
}


impl CharClassExpression {
  // Takes the inner content of square brackets, so for [a-z], send "a-z".
  fn new( contents: &str ) -> CharClassExpression {
    fn rangeAtIndex( index: uint, chars: &[char] ) -> Option<( char, char )> {
      match ( chars.get( index ),
              chars.get( index + 1 ),
              chars.get( index + 2 ) ) {
        ( Some( char1 ), Some( char2 ), Some( char3 ) ) if *char2 == '-' =>
            Some( ( *char1, *char3 ) ),
        _ => None
      }
    }

    let chars: ~[ char ] = contents.chars().collect();
    let mut char_class = CharClassExpression { single_chars: ~[],
                                               ranges: ~[] };
    let mut index = 0;
    loop {
      match rangeAtIndex( index, chars ) {
        Some( range ) => {
          char_class.ranges.push( range );
          index += 3;
        },
        _ => {
          match chars.get( index ) {
            Some( character ) => char_class.single_chars.push( *character ),
            _ => break
          };
          index += 1;
        }
      };
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
                            node: Node {
                               name: CHAR_CLASS_EXPRESSION,
                               start: index,
                               end: index + 1,
                               contents: Text( from_char( ch ) ) } } )
      }
      _ => None
    }
  }
}


struct NotExpression {
  expr: ~Expression
}


impl NotExpression {
  fn new( expr: ~Expression ) -> NotExpression {
    NotExpression { expr: expr }
  }
}


impl Expression for NotExpression {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    match self.expr.apply( parse_state ) {
      Some( _ ) => None,
      _ => Some( ParseResult { parse_state: *parse_state,
                               node: Node::predicate( NOT_EXPRESSION ) } )
    }
  }
}


fn MoveForward<B, T: Iterator<B> >( mut iter: T, steps: uint ) -> T {
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
