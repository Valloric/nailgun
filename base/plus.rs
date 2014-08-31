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

macro_rules! plus( ( $ex:expr ) => ( {
    use base;
    base::Plus::new( & $ex ) } ); )

pub struct Plus<'a> {
  expr: &'a Expression + 'a
}


impl<'a> Plus<'a> {
  pub fn new( expr: &'a Expression ) -> Plus<'a> {
    Plus { expr: expr }
  }
}


impl<'a> Expression for Plus<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    let mut final_result = ParseResult::fromParseState( *parse_state );
    let mut num_matches = 0u;
    loop {
      match self.expr.apply( &final_result.parse_state ) {
        Some( result ) => {
          final_result.parse_state = result.parse_state;
          final_result.nodes.push_all_move( result.nodes );
          num_matches += 1;
        }
        _ => break
      }
    }

    if num_matches > 0 {
      Some( final_result )
    } else {
      None
    }
  }
}


#[cfg(test)]
mod tests {
  use base::{Node, ParseResult, Expression, Data};

  #[test]
  fn Plus_Match() {
    let orig_state = input_state!( "aaa" );
    match plus!( lit!( "a" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( nodes[ 0 ],
                    Node::withoutName( 0, 1, Data( b"a" ) ) );
        assert_eq!( nodes[ 1 ],
                    Node::withoutName( 1, 2, Data( b"a" ) ) );
        assert_eq!( nodes[ 2 ],
                    Node::withoutName( 2, 3, Data( b"a" ) ) );
        assert_eq!( parse_state, orig_state.advanceTo( 3 ) );
      }
      _ => fail!( "No match." )
    }
  }

  #[test]
  fn Plus_Match_JustOne() {
    let orig_state = input_state!( "abb" );
    match plus!( lit!( "a" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( nodes[ 0 ],
                    Node::withoutName( 0, 1, Data( b"a" ) ) );
        assert_eq!( parse_state, orig_state.advanceTo( 1 ) );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn Plus_NoMatch() {
    let orig_state = input_state!( "y" );
    match plus!( lit!( "x" ) ).apply( &orig_state ) {
      None => (),
      _ => fail!( "Should not match." ),
    }
  }
}

