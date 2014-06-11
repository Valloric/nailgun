#![feature(macro_rules)]
#![allow(non_snake_case_functions)]

extern crate getopts;
extern crate inlined_parser;

use getopts::{optflag, getopts, optopt};
use std::os;
use std::io::File;
use std::path::Path;
use self::prelude::PRELUDE;
use inlined_parser::parse;

// macro_escape makes macros from annotated module visible in the "super"
// module... and thus in the children of the "super" module as well.
#[macro_escape]
mod macros;
mod generator;
mod prelude;


fn inputFromFile( input_file: &str ) -> Vec<u8> {
  match File::open( &Path::new( input_file ) ).read_to_end() {
    Ok( x ) => x,
    _ => fail!( "Couldn't read input file: {}", input_file )
  }
}


fn indentLines( input: &str, num_spaces: uint ) -> String {
  let indent = " ".repeat( num_spaces );
  input.split( '\n' ).map(
    |x| [ indent.as_slice(), x, "\n" ].concat() )
    .collect::<Vec<String>>().concat()
}


fn printUsage( opts: &[getopts::OptGroup] ) {
  let program = Path::new( os::args().get( 0 ).as_slice() );
  let short = getopts::short_usage( program.filename_str().unwrap(), opts );
  let usage = getopts::usage( short.as_slice(), opts );
  println!( "{}", usage );
}


fn printCode( input: &[u8] ) {
  match parse( input ) {
    Some( ref node ) => {
      let parse_rules = indentLines( generator::codeForNode( node ).as_slice(),
                                     2 );
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
    optopt( "p", "print-tree", "Print parsed tree.", "FILE" ),
    optopt( "g", "generate", "Generate parser code for given PEG grammar.",
            "FILE" )

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
    match parse( inputFromFile(
        matches.opt_str( "p" ).unwrap().as_slice() ).as_slice() ) {
      Some( ref node ) => println!( "{}", node ),
      _ => println!( "Couldn't parse input." )
    };
    return;
  }

  if matches.opt_present( "g" ) {
    printCode( inputFromFile(
        matches.opt_str( "g" ).unwrap().as_slice() ).as_slice() );
    return;
  }
}

