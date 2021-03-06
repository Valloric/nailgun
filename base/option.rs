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

macro_rules! opt( ( $ex:expr ) => ( &base::OptionEx::new( $ex ) ); );

pub struct OptionEx<'a> {
  expr: &'a ( Expression + 'a )
}


impl<'a> OptionEx<'a> {
  pub fn new( expr: &Expression ) -> OptionEx {
    OptionEx { expr: expr }
  }
}


impl<'b> Expression for OptionEx<'b> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    self.expr.apply( parse_state ).or(
      Some( ParseResult::fromParseState( *parse_state ) ) )
  }
}


#[cfg(test)]
mod tests {
  use base;
  use base::{Node, ParseResult, Expression, Data};

  #[test]
  fn OptionEx_Match_WithLiteral() {
    let orig_state = input_state!( "foo" );
    match opt!( lit!( "foo" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes, parse_state } ) => {
        assert_eq!( nodes[ 0 ],
                    Node::withoutName( 0, 3, Data( b"foo" ) ) );
        assert_eq!( parse_state, orig_state.advanceTo( 3 ) );
      }
      _ => panic!( "No match." )
    }
  }

  #[test]
  fn OptionEx_Match_Empty() {
    let orig_state = input_state!( "y" );
    match opt!( lit!( "x" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes, parse_state } ) => {
        assert!( nodes.is_empty() );
        assert_eq!( parse_state, orig_state );
      }
      _ => panic!( "No match." )
    }
  }
}


