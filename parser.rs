#[feature(macro_rules)];

use base::{ParseState, ParseResult, NotEx, Dot, Expression};

// macro_escape makes macros from annotated module visible in the "super"
// module... and thus in the children of the "super" module as well.
#[macro_escape]
mod macros;
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

rule!( EndOfFile <- NotEx::new( &Dot ) )

#[cfg(test)]
mod tests {
  use base::test_utils::ToParseState;
  use super::{EndOfFile};

  #[test]
  fn EndOfFile_Works() {
    assert!( EndOfFile( &ToParseState( bytes!( "" ) ) ).is_some() );
    assert!( EndOfFile( &ToParseState( bytes!( "a" ) ) ).is_none() );
  }
}
