use super::{Expression, ParseState, ParseResult, Rule};

macro_rules! ex( ( $ex:expr ) => ( {
    use base;
    base::WrapEx{ rule: $ex } } ); )

pub struct WrapEx {
  pub rule: Rule
}


impl Expression for WrapEx {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    (self.rule)( parse_state )
  }
}


#[cfg(test)]
mod tests {
  use base::{ParseResult, Expression, ParseState};
  use super::{WrapEx};

  fn advancesToOne<'a>( parse_state: &ParseState<'a> )
      -> Option< ParseResult<'a> > {
    Some( ParseResult::fromParseState( parse_state.advanceTo( 1 ) ) )
  }

  #[test]
  fn WrapEx_ReturnsSome() {
    assert!( ex!( advancesToOne ).apply( &input_state!( b"foo" ) ).is_some() );
  }
}


