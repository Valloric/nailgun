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

macro_rules! and( ( $ex:expr ) => ( {
    use base;
    &base::And::new( $ex ) } ); )

pub struct And<'a> {
  expr: &'a Expression + 'a
}


impl<'a> And<'a> {
  pub fn new( expr: &'a Expression ) -> And<'a> {
    And { expr: expr }
  }
}


impl<'a> Expression for And<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    match self.expr.apply( parse_state ) {
      Some( _ ) => Some( ParseResult::fromParseState( *parse_state ) ),
      _ => None
    }
  }
}


#[cfg(test)]
mod tests {
  use base::{ParseResult, Expression};

  #[test]
  fn And_Match_WithLiteral() {
    let orig_state = input_state!( "foo" );
    match and!( lit!( "foo" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert!( nodes.is_empty() );
        assert_eq!( parse_state, orig_state );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn And_Match_WithCharClass() {
    let orig_state = input_state!( "c" );
    match and!( class!( "a-z" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert!( nodes.is_empty() );
        assert_eq!( parse_state, orig_state );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn And_NoMatch() {
    assert!( and!( class!( "a-z" ) ).apply( &input_state!( "0" ) ).is_none() )
    assert!( and!( lit!( "x" ) ).apply( &input_state!( "y" ) ).is_none() )
  }
}

