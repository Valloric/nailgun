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
use base::unicode::{bytesFollowing, readCodepoint};

pub struct Dot;
impl Expression for Dot {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) -> Option< ParseResult<'a> > {
    match readCodepoint( parse_state.input ) {
      Some( _ ) => {
        let num_following = bytesFollowing( parse_state.input[ 0 ] ).unwrap();
        return parse_state.offsetToResult(
          parse_state.offset + num_following + 1 )
      }
      _ => ()
    }

    match parse_state.input.get( 0 ) {
      Some( _ ) => parse_state.offsetToResult( parse_state.offset + 1 ),
      _ => None
    }
  }
}


#[cfg(test)]
mod tests {
  use super::{Dot};
  use base::{Node, Data, ParseResult, ParseState, Expression};

  #[test]
  fn Dot_Match_InputOneChar() {
    match Dot.apply( &input_state!( "x" ) ) {
      Some( ParseResult{ nodes, parse_state } ) => {
        assert_eq!( nodes[ 0 ],
                    Node::withoutName( 0, 1, Data( b"x" ) ) );
        assert_eq!( parse_state, ParseState{ input: &[], offset: 1 } );
      }
      _ => panic!( "No match!" )
    };
  }


  #[test]
  fn Dot_Match_InputOneWideChar() {
    match Dot.apply( &input_state!( "葉" ) ) {
      Some( ParseResult{ nodes, parse_state } ) => {
        assert_eq!( nodes[ 0 ],
                    Node::withoutName( 0, 3, Data( "葉".as_bytes() ) ) );
        assert_eq!( parse_state, ParseState{ input: &[], offset: 3 } );
      }
      _ => panic!( "No match!" )
    };
  }


  #[test]
  fn Dot_Match_InputSeveralChars() {
    match Dot.apply( &input_state!( "xb" ) ) {
      Some( ParseResult{ nodes, parse_state } ) => {
        assert!( nodes[ 0 ] ==
                 Node::withoutName( 0, 1, Data( b"x" ) ) );
        assert_eq!( parse_state, ParseState{ input: b"b",
                                             offset: 1 } );
      }
      _ => panic!( "No match!" )
    };
  }


  #[test]
  fn Dot_NoMatch() {
    assert!( Dot.apply( &input_state!( "" ) ).is_none() )
  }
}
