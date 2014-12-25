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

macro_rules! seq( ( $( $ex:expr ),* ) => ( {
    use base;
    &base::Sequence::new( &[ $( $ex ),* ] ) } ); );

pub struct Sequence<'a> {
  exprs: &'a [&'a (Expression + 'a)]
}


impl<'b> Sequence<'b> {
  pub fn new<'a>( exprs: &'a [&Expression] ) -> Sequence<'a> {
    Sequence { exprs: exprs }
  }
}


impl<'b> Expression for Sequence<'b> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    let mut final_result = ParseResult::fromParseState( *parse_state );
    for expr in self.exprs.iter() {
      match expr.apply( &final_result.parse_state ) {
        Some( result ) => {
          final_result.parse_state = result.parse_state;
          final_result.nodes.extend( result.nodes.into_iter() );
        }
        _ => return None
      }
    }
    Some( final_result )
  }
}


#[cfg(test)]
mod tests {
  use base::{Node, ParseResult, Expression, Data};

  #[test]
  fn Sequence_Match() {
    let orig_state = input_state!( "ab" );
    match seq!( lit!( "a" ), lit!( "b" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes, parse_state } ) => {
        assert_eq!( nodes[ 0 ],
                    Node::withoutName( 0, 1, Data( b"a" ) ) );
        assert_eq!( nodes[ 1 ],
                    Node::withoutName( 1, 2, Data( b"b" ) ) );
        assert_eq!( parse_state, orig_state.advanceTo( 2 ) );
      }
      _ => panic!( "No match." )
    }
  }

  #[test]
  fn Sequence_NoMatch() {
    assert!( seq!( lit!( "a" ), lit!( "b" ) ).apply(
        &input_state!( "aa" ) ).is_none() );
  }
}
