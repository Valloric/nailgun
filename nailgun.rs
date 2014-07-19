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
#![feature(macro_rules)]
#![allow(non_snake_case_functions)]

extern crate getopts;
extern crate inlined_parser;

use getopts::{optflag, getopts, optopt};
use std::os;
use std::io::File;
use std::io::TempDir;
use std::io::Command;
use std::io::process::ExitStatus;
use std::path::Path;
use self::prelude::PRELUDE;
use self::printer::PRINTER_MAIN;
use inlined_parser::{parse, Node};
use std::str;

mod generator;
mod prelude;
mod printer;

static TOP_LEVEL_RULE : &'static str = "NGTOP_LEVEL_RULE";


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
  let program = Path::new( os::args()[ 0 ].as_slice() );
  let short = getopts::short_usage( program.filename_str().unwrap(), opts );
  let usage = getopts::usage( short.as_slice(), opts );
  println!( "{}", usage );
}


fn nameOfFirstRule<'a>( root: &'a Node<'a> ) -> String {
  str::from_utf8(
    root.preOrder().find( |x| x.name == "Definition" ).unwrap()
        .preOrder().find( |x| x.name == "Identifier" ).unwrap()
        .matchedData().as_slice() ).unwrap().trim_chars(' ').to_string()
}


fn codeForGrammar( input: &[u8] ) -> Option<String> {
  match parse( input ) {
    Some( ref node ) => {
      let parse_rules = indentLines( generator::codeForNode( node ).as_slice(),
                                     2 );
      let prepared_prelude = str::replace(
        PRELUDE.slice_to( PRELUDE.len() -1 ),
        TOP_LEVEL_RULE,
        nameOfFirstRule( node ).as_slice() );

      Some( [ prepared_prelude.as_slice(),
              "\n",
              parse_rules.as_slice(),
              "}" ].concat() )
    }
    _ => None
  }
}


fn printParseTree( grammar_code: &str, input_path: &str ) {
  let final_code = grammar_code.to_string().append( PRINTER_MAIN );
  let temp_dir = TempDir::new( "temp" ).unwrap();
  let code_file = temp_dir.path().join( "printer.rs" );
  let printer = temp_dir.path().join( "printer" );

  match File::create( &code_file ).write( final_code.as_bytes() ) {
    Err( e ) => fail!( "File error: {}", e ),
    _ => {}
  };

  match Command::new( "rustc" ).arg( "-o" )
                               .arg( printer.as_str().unwrap() )
                               .arg( code_file.as_str().unwrap() )
                               .status() {
    Ok( status ) if !status.success() =>
      fail!( "Compiling with rustc failed." ),
    Err( e ) => fail!( "Failed to execute process: {}", e ),
    _ => {}
  };

  let printer = temp_dir.path().join( "printer" );
  let command_output = Command::new(
      printer.as_str().unwrap() ).arg( input_path ).output();

  match command_output {
    Ok( output ) => {
      println!( "{}", String::from_utf8_lossy( output.output.as_slice() ) );
      os::set_exit_status( match output.status {
        ExitStatus( code ) => code,
        _ => 1
      } );
    },
    Err( e ) => fail!( "Failed to execute process: {}", e ),
  };
}


#[cfg(not(test))]
fn main() {
  let opts = [
    optflag( "h", "help", "Print this help menu." ),
    optopt( "i", "input",
            "Path to input file to parse with grammar given to -g option",
            "FILE" ),
    optopt( "g", "grammar",
            "Path to PEG grammar. Prints code for grammar if -i not given.",
            "FILE" )

  ];

  let args = os::args();
  let matches = match getopts( args.tail(), opts ) {
    Ok( m ) => m,
    Err( erorr ) => fail!( erorr )
  };

  if matches.opt_present( "h" ) || args.len() < 2 {
    printUsage( opts );
    return;
  }

  let grammar_code = if matches.opt_present( "g" ) {
    codeForGrammar( inputFromFile(
        matches.opt_str( "g" ).unwrap().as_slice() ).as_slice() )
    .unwrap_or_else( || fail!( "Couldn't parse given PEG grammar" ) )
  } else {
    fail!( "Missing -g option." )
  };


  if matches.opt_present( "i" ) {
    printParseTree(
      grammar_code.as_slice(),
      matches.opt_str( "i" ).unwrap().as_slice() );
  } else {
    println!( "{}", grammar_code );
  }
}

