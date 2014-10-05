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

macro_rules! fuse( ( $ex:expr ) => ( {
    use base;
    &base::Fuse::new( $ex ) } ); )

pub struct Fuse<'a> {
  expr: &'a Expression + 'a
}


impl<'a> Fuse<'a> {
  pub fn new<'a>( expr: &'a Expression ) -> Fuse<'a> {
    Fuse { expr: expr }
  }
}


impl<'a> Expression for Fuse<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    match self.expr.apply( parse_state ) {
      Some( result ) => {
        // NOTE: This will need a more complex implementation when we implement
        // the prune (:) operator.
        parse_state.offsetToResult( result.parse_state.offset )
      },
      _ => None
    }
  }
}


#[cfg(test)]
mod tests {
  use base::{Node, Data, ParseResult, Expression};

  #[test]
  fn Fuse_Match_WithLiteralStar() {
    let orig_state = input_state!( "ooo" );
    match fuse!( plus!( lit!( "o" ) ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( nodes[ 0 ],
                    Node::withoutName( 0, 3, Data( b"ooo" ) ) );
        assert_eq!( parse_state, orig_state.advanceTo( 3 ) );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn Fuse_Match_WithCharClass() {
    let orig_state = input_state!( "abc" );
    match fuse!( plus!( class!( "a-z" ) ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes: nodes,
                         parse_state: parse_state } ) => {
        assert_eq!( nodes[ 0 ],
                    Node::withoutName( 0, 3, Data( b"abc" ) ) );
        assert_eq!( parse_state, orig_state.advanceTo( 3 ) );
      }
      _ => fail!( "No match." )
    }
  }


  #[test]
  fn Fuse_NoMatch() {
    assert!( fuse!( class!( "a-z" ) ).apply( &input_state!( "5" ) ).is_none() )
    assert!( fuse!( lit!( "x" ) ).apply( &input_state!( "g" ) ).is_none() )
  }
}

