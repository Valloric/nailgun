use base::ParseState;

pub fn ToParseState<'a>( bytes: &'a [u8] ) -> ParseState<'a> {
  ParseState { input: bytes, offset: 0 }
}

macro_rules! input_state( ( $ex:expr ) => ( {
      use base::ParseState;
      byte_var!( input = $ex )
      ParseState { input: input, offset: 0 }
    } ) )
