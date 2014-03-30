#![feature(macro_rules)]

extern crate parser;
use std::io;

#[cfg(not(test))]
fn main() {
  let data = io::stdin().read_to_end().unwrap();

  println!( "{:?}", parser::parse( data ) );
}

