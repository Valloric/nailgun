// Copyright 2014 Strahinja Val Markovic
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use super::{Expression, ParseState, ParseResult, Rule};

macro_rules! ex( ( $ex:expr ) => ( &base::WrapEx{ rule: $ex } ); );

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
  use base;
  use base::{ParseResult, Expression, ParseState};

  fn advancesToOne<'a>( parse_state: &ParseState<'a> )
      -> Option< ParseResult<'a> > {
    Some( ParseResult::fromParseState( parse_state.advanceTo( 1 ) ) )
  }

  #[test]
  fn WrapEx_ReturnsSome() {
    assert!( ex!( advancesToOne ).apply( &input_state!( "foo" ) ).is_some() );
  }
}


