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

macro_rules! or( ( $( $ex:expr ),* ) => ( {
    use base;
    &base::Or::new( &[ $( $ex ),* ] ) } ); )

pub struct Or<'a> {
  exprs: &'a [&'a Expression + 'a]
}


impl<'a> Or<'a> {
  pub fn new<'a>( exprs: &'a [&Expression] ) -> Or<'a> {
    Or { exprs: exprs }
  }
}


impl<'a> Expression for Or<'a> {
  fn apply<'a>( &self, parse_state: &ParseState<'a> ) ->
      Option< ParseResult<'a> > {
    for expr in self.exprs.iter() {
      match expr.apply( parse_state ) {
        result @ Some( _ ) => return result,
        _ => ()
      }
    }
    None
  }
}


#[cfg(test)]
mod tests {
  use base::{Node, ParseResult, Expression, Data};

  #[test]
  fn Or_Match_FirstExpr() {
    let orig_state = input_state!( "a" );
    match or!( lit!( "a" ), lit!( "b" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes, parse_state } ) => {
        assert_eq!( nodes[ 0 ],
                    Node::withoutName( 0, 1, Data( b"a" ) ) );
        assert_eq!( parse_state, orig_state.advanceTo( 1 ) );
      }
      _ => panic!( "No match." )
    }
  }

  #[test]
  fn Or_Match_SecondExpr() {
    let orig_state = input_state!( "a" );
    match or!( lit!( "b" ), lit!( "a" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes, parse_state } ) => {
        assert_eq!( nodes[ 0 ],
                    Node::withoutName( 0, 1, Data( b"a" ) ) );
        assert_eq!( parse_state, orig_state.advanceTo( 1 ) );
      }
      _ => panic!( "No match." )
    }
  }

  #[test]
  fn Or_Match_FirstExprIfBoth() {
    let orig_state = input_state!( "a" );
    match or!( lit!( "a" ), lit!( "a" ) ).apply( &orig_state ) {
      Some( ParseResult{ nodes, parse_state } ) => {
        assert_eq!( nodes[ 0 ],
                    Node::withoutName( 0, 1, Data( b"a" ) ) );
        assert_eq!( parse_state, orig_state.advanceTo( 1 ) );
      }
      _ => panic!( "No match." )
    }
  }

  #[test]
  fn Or_NoMatch() {
    assert!( or!( lit!( "b" ), lit!( "c" ) ).apply(
        &input_state!( "a" ) ).is_none() )
  }
}

