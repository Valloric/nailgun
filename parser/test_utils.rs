use parser::ParseState;

pub fn ToParseState<'a>( text: &'a str ) -> ParseState<'a> {
  text.chars().enumerate()
}
