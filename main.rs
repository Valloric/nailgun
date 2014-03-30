#![feature(macro_rules)]

extern crate parser;
use std::io;

#[cfg(not(test))]
fn main() {
  let input = io::stdin().read_to_end().unwrap();
  match parser::parse( input ) {
    Some( ref node ) => println!( "{}", node ),
    _ => println!( "Couldn't parse input." )
  }
}

