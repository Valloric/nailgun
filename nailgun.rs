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
#![feature(path)]
#![feature(old_path)]
#![feature(old_io)]
#![feature(os)]
#![feature(env)]
#![feature(collections)]
#![feature(core)]
#![feature(unicode)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![deny(deprecated)]

extern crate getopts;
extern crate inlined_parser;

use getopts::Options;
use std::env;
use std::old_io::File;
use std::old_io::TempDir;
use std::old_io::Command;
use std::old_io::process::ExitStatus;
use std::path::Path;
use std::old_path;
use self::prelude::PRELUDE;
use self::printer::PRINTER_MAIN;
use inlined_parser::{parse, Node};
use std::str;
use std::iter::repeat;

mod generator;
mod prelude;
mod printer;

static TOP_LEVEL_RULE : &'static str = "NGTOP_LEVEL_RULE";


fn inputFromFile( input_file: &str ) -> Vec<u8> {
  match File::open( &old_path::Path::new( input_file ) ).read_to_end() {
    Ok( x ) => x,
    _ => panic!( "Couldn't read input file: {}", input_file )
  }
}


fn indentLines( input: &str, num_spaces: usize ) -> String {
  let indent: String = repeat( " " ).take( num_spaces ).collect();
  input.split( '\n' ).map(
    |x| [ &indent[..], x, "\n" ].concat() )
    .collect::<Vec<String>>().concat()
}


fn printUsage( opts: &getopts::Options ) {
  let program_path = env::args().next().unwrap();
  let program = Path::new( &program_path );
  let short = opts.short_usage( program.file_name().unwrap().to_str().unwrap() );
  let usage = opts.usage( &short[..] );
  println!( "{}", usage );
}


fn nameOfFirstRule<'a>( root: &'a Node<'a> ) -> String {
  str::from_utf8(
    &root.preOrder().find( |x| x.name == "Identifier" ).unwrap()
        .matchedData()[..] ).unwrap().trim_matches(' ').to_string()
}


fn codeForGrammar( input: &[u8] ) -> Option<String> {
  match parse( input ) {
    Some( ref node ) => {
      let parse_rules = indentLines( &generator::codeForNode( node )[..], 2 );
      let prepared_prelude = PRELUDE[ .. PRELUDE.len() -1 ].replace(
        TOP_LEVEL_RULE,
        &nameOfFirstRule( node )[..] );

      Some( [ &prepared_prelude[..],
              "\n",
              &parse_rules[..],
              "}" ].concat() )
    }
    _ => None
  }
}


fn printParseTree( grammar_code: &str, input_path: &str ) {
  let mut final_code = grammar_code.to_string();
  final_code.push_str( PRINTER_MAIN );
  let temp_dir = TempDir::new( "temp" ).unwrap();
  let code_file = temp_dir.path().join( "printer.rs" );
  let printer = temp_dir.path().join( "printer" );

  match File::create( &code_file ).write_all( final_code.as_bytes() ) {
    Err( e ) => panic!( "File error: {}", e ),
    _ => {}
  };

  match Command::new( "rustc" ).arg( "-o" )
                               .arg( printer.as_str().unwrap() )
                               .arg( code_file.as_str().unwrap() )
                               .status() {
    Ok( status ) if !status.success() =>
      panic!( "Compiling with rustc failed." ),
    Err( e ) => panic!( "Failed to execute process: {}", e ),
    _ => {}
  };

  let printer = temp_dir.path().join( "printer" );
  let command_output = Command::new(
      printer.as_str().unwrap() ).arg( input_path ).output();

  match command_output {
    Ok( output ) => {
      println!( "{}", String::from_utf8_lossy( &output.output[..] ) );
      env::set_exit_status( match output.status {
        ExitStatus( code ) => code as i32,
        _ => 1
      } );
    },
    Err( e ) => panic!( "Failed to execute process: {}", e ),
  };
}


#[cfg(not(test))]
fn main() {
  let mut opts = Options::new();
  opts.optflag( "h", "help", "Print this help menu." );
  opts.optopt( "i", "input",
               "Path to input file to parse with grammar given to -g option",
               "FILE" );
  opts.optopt( "g", "grammar",
               "Path to PEG grammar. Prints code for grammar if -i not given.",
               "FILE" );

  let args: Vec<_> = env::args().collect();
  let matches = match opts.parse( args.tail() ) {
    Ok( m ) => m,
    Err( erorr ) => panic!( erorr )
  };

  if matches.opt_present( "h" ) || args.len() < 2 {
    printUsage( &opts );
    return;
  }

  let grammar_code = if matches.opt_present( "g" ) {
    codeForGrammar( &inputFromFile(
        &matches.opt_str( "g" ).unwrap()[..] )[..] )
    .unwrap_or_else( || panic!( "Couldn't parse given PEG grammar" ) )
  } else {
    panic!( "Missing -g option." )
  };


  if matches.opt_present( "i" ) {
    printParseTree(
      &grammar_code[..],
      &matches.opt_str( "i" ).unwrap()[..] );
  } else {
    println!( "{}", grammar_code );
  }
}

