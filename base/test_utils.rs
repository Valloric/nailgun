use base::ParseState;

pub fn ToParseState<'a>( bytes: &'a [u8] ) -> ParseState<'a> {
  ParseState { input: bytes, offset: 0 }
}

macro_rules! input_state( ( $ex:expr ) => ( {
      use base::ParseState;
      use std::str::StrSlice;
      ParseState { input: $ex.as_bytes(), offset: 0 }
    } ) )
