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

macro_rules! lit( ( $ex:expr ) => ( {
      use base;
      use std::str::StrExt;
      &base::Literal::new( $ex.as_bytes() ) } ) );


pub struct Literal {
  text: &'static [u8]
}


impl Literal {
  pub fn new( text: &'static [u8] ) -> Literal {
    Literal { text: text }
  }
}


impl Expression for Literal {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    if parse_state.input.len() < self.text.len() ||
       &parse_state.input[ .. self.text.len() ] != self.text {
      return None;
    }

    parse_state.offsetToResult( parse_state.offset + self.text.len() )
  }
}


#[cfg(test)]
mod tests {
  use base::{Node, Data, ParseResult, ParseState, Expression};

  #[test]
  fn Literal_Match() {
    let expr = lit!( "foo" );
    match expr.apply( &input_state!( "foobar" ) ) {
      Some( ParseResult{ nodes, parse_state } ) => {
        assert_eq!( nodes[ 0 ],
                    Node::withoutName( 0, 3, Data( b"foo" ) ) );
        assert_eq!( parse_state, ParseState{ input: b"bar",
                                             offset: 3 } );
      }
      _ => panic!( "No match!" )
    };
  }


  #[test]
  fn Literal_NoMatch() {
    let expr = lit!( "zoo" );
    assert!( expr.apply( &input_state!( "foobar" ) ).is_none() );
    assert!( expr.apply( &input_state!( "" ) ).is_none() );
  }
}
