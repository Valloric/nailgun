use base::ParseState;

pub fn ToParseState<'a>( bytes: &'a [u8] ) -> ParseState<'a> {
  ParseState { input: bytes, offset: 0 }
}

macro_rules! input_state( ( $ex:expr ) => ( {
      use base::ParseState;
      ParseState { input: $ex, offset: 0 }
    } ) )
