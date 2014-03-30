#[feature(macro_rules)];

use base::{ParseState, ParseResult, Dot, Expression};

// macro_escape makes macros from annotated module visible in the "super"
// module... and thus in the children of the "super" module as well.
#[macro_escape]
mod macros;

#[macro_escape]
mod base;

macro_rules! rule(
  (
    $name:ident <- $body:expr
  ) => (
    fn $name<'a>( parse_state: &ParseState<'a> ) -> Option< ParseResult<'a> > {
      $body.apply( parse_state )
    }
  );
)


rule!( Literal <- or!( seq!( class!( "'" ),
                             star!( seq!( not!( class!( "'" ) ),
                                          ex!( Char ) ) ),
                             class!( "'" ),
                             ex!( Spacing ) ),
                       seq!( class!( "\"" ),
                             star!( seq!( not!( class!( "\"" ) ),
                                          ex!( Char ) ) ),
                             class!( "\"" ),
                             ex!( Spacing ) ) ) )
rule!( Class <- seq!( lit!( "[" ),
                      star!( seq!( not!( lit!( "]" ) ), ex!( Range ) ) ),
                      lit!( "]" ),
                      ex!( Spacing ) ) )
rule!( Range <- or!( seq!( ex!( Char ), lit!( "-" ), ex!( Char ) ),
                     ex!( Char ) ) )
rule!( Char <- or!( seq!( lit!( r"\" ),
                          class!( r#"nrt'"[]\"# ) ),
                    seq!( lit!( r"\" ),
                          class!( "0-2" ),
                          class!( "0-7" ),
                          class!( "0-7" ) ),
                    seq!( lit!( r"\" ),
                          class!( "0-7" ),
                          opt!( class!( "0-7" ) ) ),
                    seq!( not!( lit!( r"\" ) ),
                          Dot ) ) )
rule!( LEFTARROW <- seq!( lit!( "<-" ), ex!( Spacing ) ) )
rule!( SLASH <- seq!( lit!( "/" ), ex!( Spacing ) ) )
rule!( NOT <- seq!( lit!( "!" ), ex!( Spacing ) ) )
rule!( QUESTION <- seq!( lit!( "?" ), ex!( Spacing ) ) )
rule!( STAR <- seq!( lit!( "*" ), ex!( Spacing ) ) )
rule!( PLUS <- seq!( lit!( "+" ), ex!( Spacing ) ) )
rule!( OPEN <- seq!( lit!( "(" ), ex!( Spacing ) ) )
rule!( CLOSE <- seq!( lit!( ")" ), ex!( Spacing ) ) )
rule!( DOT <- seq!( lit!( "." ), ex!( Spacing ) ) )
rule!( Spacing <- star!( or!( ex!( Space ), ex!( Comment ) ) ) )
rule!( Comment <- seq!( lit!( "#" ),
                        star!( seq!( not!( ex!( EndOfLine ) ), Dot ) ),
                        ex!( EndOfLine ) ) )
rule!( Space <- or!( lit!( " " ), lit!( "\t" ), ex!( EndOfLine ) ) )
rule!( EndOfLine <- or!( lit!( "\r\n" ), lit!( "\n" ), lit!( "\r" ) ) )
rule!( EndOfFile <- not!( Dot ) )


#[cfg(test)]
mod tests {
  use base::test_utils::ToParseState;
  use base::{ParseResult};
  use super::{EndOfFile, EndOfLine, Space, Comment, Spacing, Char, Range, Class,
              Literal};

  macro_rules! consumes(
    (
      $name:ident, $input:expr
    ) => (
      {
        byte_var!( input = $input )
        match $name( &ToParseState( input ) ) {
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
      {
        byte_var!( input = $input )
        $name( &ToParseState( input ) ).is_some()
      }
    );
  )

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

    // Spacing DOES match here because at the top level, it is a star expression
    // which can match consuming nothing.
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
