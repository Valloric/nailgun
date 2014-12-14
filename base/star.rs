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

macro_rules! star( ( $ex:expr ) => ( {
    use base;
    &base::Star::new( $ex ) } ); )

pub struct Star<'a> {
  expr: &'a ( Expression + 'a )
}


impl<'a> Star<'a> {
  pub fn new<'a>( expr: &'a Expression ) -> Star<'a> {
    Star { expr: expr }
  }
}


impl<'a> Expression for Star<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    let mut final_result = ParseResult::fromParseState( *parse_state );
    loop {
      match self.expr.apply( &final_result.parse_state ) {
        Some( result ) => {
          final_result.parse_state = result.parse_state;
          final_result.nodes.extend( result.nodes.into_iter() );
        }
        _ => break
      }
    }
    Some( final_result )
  }
}


#[cfg(test)]
mod tests {
  use base::{Node, ParseResult, Expression, Data};

  #[test]
  fn Star_Match() {
    let orig_state = input_state!( "aaa" );
    match star!( lit!( "a" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes, parse_state } ) => {
        assert_eq!( nodes[ 0 ],
                    Node::withoutName( 0, 1, Data( b"a" ) ) );
        assert_eq!( nodes[ 1 ],
                    Node::withoutName( 1, 2, Data( b"a" ) ) );
        assert_eq!( nodes[ 2 ],
                    Node::withoutName( 2, 3, Data( b"a" ) ) );
        assert_eq!( parse_state, orig_state.advanceTo( 3 ) );
      }
      _ => panic!( "No match." )
    }
  }

  #[test]
  fn Star_Match_JustOne() {
    let orig_state = input_state!( "abb" );
    match star!( lit!( "a" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes, parse_state } ) => {
        assert_eq!( nodes[ 0 ],
                    Node::withoutName( 0, 1, Data( b"a" ) ) );
        assert_eq!( parse_state, orig_state.advanceTo( 1 ) );
      }
      _ => panic!( "No match." )
    }
  }


  #[test]
  fn Star_Match_Empty() {
    let orig_state = input_state!( "y" );
    match star!( lit!( "x" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes, parse_state } ) => {
        assert!( nodes.is_empty() );
        assert_eq!( parse_state, orig_state );
      }
      _ => panic!( "No match." )
    }
  }
}
