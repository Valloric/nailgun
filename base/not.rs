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
use super::{Expression, ParseState, ParseResult};

macro_rules! not( ( $ex:expr ) => ( {
    use base;
    &base::NotEx::new($ex) } ); )

pub struct NotEx<'a> {
  expr: &'a Expression + 'a
}


impl<'a> NotEx<'a> {
  pub fn new<'a>( expr: &'a Expression ) -> NotEx<'a> {
    NotEx { expr: expr }
  }
}


impl<'a> Expression for NotEx<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    match self.expr.apply( parse_state ) {
      Some( _ ) => None,
      _ => Some( ParseResult::fromParseState( *parse_state ) )
    }
  }
}


#[cfg(test)]
mod tests {
  use base::{ParseResult, Expression};

  #[test]
  fn NotEx_Match_WithLiteral() {
    let orig_state = input_state!( "zoo" );
    match not!( lit!( "foo" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes, parse_state } ) => {
        assert!( nodes.is_empty() );
        assert_eq!( parse_state, orig_state );
      }
      _ => panic!( "No match." )
    }
  }


  #[test]
  fn NotEx_Match_WithCharClass() {
    let orig_state = input_state!( "0" );
    match not!( class!( "a-z" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes, parse_state } ) => {
        assert!( nodes.is_empty() );
        assert_eq!( parse_state, orig_state );
      }
      _ => panic!( "No match." )
    }
  }


  #[test]
  fn NotEx_NoMatch() {
    assert!( not!( class!( "a-z" ) ).apply( &input_state!( "b" ) ).is_none() )
    assert!( not!( lit!( "x" ) ).apply( &input_state!( "x" ) ).is_none() )
  }
}
