#![feature(macro_rules)]

extern crate getopts;
extern crate inlined_parser;

use getopts::{optflag, getopts};
use std::os;
use std::io;
use std::path::Path;
use self::prelude::PRELUDE;
use inlined_parser::parse;

mod generator;
mod prelude;


fn stdin() -> Vec<u8> {
  io::stdin().read_to_end().unwrap()
}


fn indentLines( input: ~str, num_spaces: uint ) -> ~str {
  let indent = " ".repeat( num_spaces );
  input.split( '\n' ).map(
    |x| [ indent.as_slice(), x, "\n" ].concat() )
    .collect::<Vec<~str>>().concat()
}


fn printUsage( opts: &[getopts::OptGroup] ) {
  let program = Path::new( os::args()[ 0 ] );
  let short = getopts::short_usage( program.filename_str().unwrap(), opts );
  let usage = getopts::usage( short, opts );
  println!( "{}", usage );
}


fn printCode() {
  match parse( stdin().as_slice() ) {
    Some( ref node ) => {
      let parse_rules = indentLines( generator::codeForNode( node ), 2 );
      let output = [ PRELUDE.slice_to( PRELUDE.len() -1 ),
                     "\n",
                     parse_rules.as_slice(),
                     "}" ].concat();
      println!( "{}", output );
    }
    _ => println!( "Couldn't parse input." )
  }
}


#[cfg(not(test))]
fn main() {
  let opts = [
    optflag( "h", "help", "Print this help menu." ),
    // TODO: Should actually take a PEG grammar file and input, and then print
    // parsed tree of input.
    optflag( "p", "print-tree", "Print parsed tree." ),
    optflag( "g", "generate", "Generate parser code for given PEG grammar." )
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
    match parse( stdin().as_slice() ) {
      Some( ref node ) => println!( "{}", node ),
      _ => println!( "Couldn't parse input." )
    }
  }

  if matches.opt_present( "g" ) {
    printCode()
  }
}

