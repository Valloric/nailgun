use base::ParseState;

pub fn ToParseState<'a>( bytes: &'a [u8] ) -> ParseState<'a> {
  ParseState { input: bytes, offset: 0 }
}

