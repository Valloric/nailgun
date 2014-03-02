use std::str::from_char;
use super::{Expression, Node, ParseState, ParseResult, Text};

static CHAR_CLASS_EXPRESSION : &'static str = "CharClassExpression";

pub struct CharClassExpression {
  // All the single chars in the char class
  single_chars: ~[ char ],

  // Sequence of [from, to] (inclusive bounds) char ranges
  ranges: ~[ ( char, char ) ]
}


impl CharClassExpression {
  // Takes the inner content of square brackets, so for [a-z], send "a-z".
  pub fn new( contents: &str ) -> CharClassExpression {
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
      Some( ( index, ch ) ) if self.matches( ch ) =>
        Some( ParseResult::oneNode(
            Node { name: CHAR_CLASS_EXPRESSION,
                   start: index,
                   end: index + 1,
                   contents: Text( from_char( ch ) ) },
            new_parse_state ) ),
      _ => None
    }
  }
}


#[cfg(test)]
mod tests {
  use parser::{Node, Text, ParseResult, Expression};
  use parser::test_utils::ToParseState;
  use super::{CHAR_CLASS_EXPRESSION, CharClassExpression};

  fn charClassMatch( char_class: CharClassExpression, input: &str ) -> bool {
    match char_class.apply( &ToParseState( input ) ) {
      Some( ParseResult{ nodes: ref nodes,
                        parse_state: mut parse_state } ) => {
        assert_eq!( *nodes.get( 0 ).unwrap(),
                    Node { name: CHAR_CLASS_EXPRESSION,
                          start: 0,
                          end: 1,
                          contents: Text( input.to_owned() ) } );
        assert_eq!( parse_state.next(), None );
        true
      }
      _ => false
    }
  }


  #[test]
  fn CharClassExpression_Match() {
    assert!( charClassMatch( CharClassExpression::new( "a"          ), "a" ) );
    assert!( charClassMatch( CharClassExpression::new( "abcdef"     ), "e" ) );
    assert!( charClassMatch( CharClassExpression::new( "a-z"        ), "a" ) );
    assert!( charClassMatch( CharClassExpression::new( "a-z"        ), "c" ) );
    assert!( charClassMatch( CharClassExpression::new( "a-z"        ), "z" ) );
    assert!( charClassMatch( CharClassExpression::new( "0-9"        ), "2" ) );
    assert!( charClassMatch( CharClassExpression::new( "α-ω"        ), "η" ) );
    assert!( charClassMatch( CharClassExpression::new( "-"          ), "-" ) );
    assert!( charClassMatch( CharClassExpression::new( "a-"         ), "-" ) );
    assert!( charClassMatch( CharClassExpression::new( "-a"         ), "-" ) );
    assert!( charClassMatch( CharClassExpression::new( "a-zA-Z-"    ), "-" ) );
    assert!( charClassMatch( CharClassExpression::new( "aa-zA-Z-a"  ), "-" ) );
    assert!( charClassMatch( CharClassExpression::new( "a-zA-Z-"    ), "z" ) );
    assert!( charClassMatch( CharClassExpression::new( "aa-zA-Z-0"  ), "0" ) );
    assert!( charClassMatch( CharClassExpression::new( "a-cdefgh-k" ), "e" ) );
    assert!( charClassMatch( CharClassExpression::new( "---"        ), "-" ) );
    assert!( charClassMatch( CharClassExpression::new( "a-a"        ), "a" ) );
  }


  #[test]
  fn CharClassExpression_NoMatch() {
    assert!( !charClassMatch( CharClassExpression::new( "a"   ), "b" ) );
    assert!( !charClassMatch( CharClassExpression::new( "-"   ), "a" ) );
    assert!( !charClassMatch( CharClassExpression::new( "z-a" ), "a" ) );
    assert!( !charClassMatch( CharClassExpression::new( "z-a" ), "b" ) );
    assert!( !charClassMatch( CharClassExpression::new( "a-z" ), "0" ) );
    assert!( !charClassMatch( CharClassExpression::new( "a-z" ), "A" ) );
  }

}
