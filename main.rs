#![feature(macro_rules)]

extern crate getopts;
extern crate parser;
use getopts::{optflag, getopts};
use std::os;
use std::io;


fn printUsage( opts: &[getopts::OptGroup] ) {
  let args = os::args();
  let short = getopts::short_usage( args[0], opts );
  let usage = getopts::usage( short, opts );
  println!( "{}", usage );
}

#[cfg(not(test))]
fn main() {
  let opts = [
    optflag( "h", "help", "Print this help menu." ),
    // TODO: Should actually take a PEG grammar file and input, and then print
    // parsed tree of input.
    optflag( "p", "print-tree", "Print parsed tree." )
  ];

  let args = os::args();
  let matches = match getopts( args.tail(), opts ) {
    Ok( m ) => m,
    Err( erorr ) => fail!( erorr.to_err_msg() )
  };

  if matches.opt_present( "h" ) {
    printUsage( opts );
    return;
  }

  if matches.opt_present( "p" ) {
    let input = io::stdin().read_to_end().unwrap();
    match parser::parse( input ) {
      Some( ref node ) => println!( "{}", node ),
      _ => println!( "Couldn't parse input." )
    }
  }
}

