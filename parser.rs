#[feature(macro_rules)];

use base::{ParseState, ParseResult, NotEx, Dot, Expression, Literal, Or};

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

rule!( EndOfLine <- or!( lit!( "\r\n" ), lit!( "\n" ), lit!( "\r" ) ) )
rule!( EndOfFile <- not!( &Dot ) )



#[cfg(test)]
mod tests {
  use base::test_utils::ToParseState;
  use base::{ParseResult};
  use super::{EndOfFile, EndOfLine};

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
