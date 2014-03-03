use parser::ParseState;

pub fn ToParseState<'a>( text: &'a str ) -> ParseState<'a> {
  text.chars().enumerate()
}


pub fn bytes( input: &'static str ) -> ~[u8] {
  input.to_owned().into_bytes()
}
