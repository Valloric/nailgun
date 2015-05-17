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
#![feature(collections)]
#![feature(unicode)]
#![feature(slice_patterns)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![deny(deprecated)]

extern crate getopts;
extern crate tempdir;
extern crate inlined_parser;

use tempdir::TempDir;
use getopts::Options;
use std::env;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::process;
use std::process::Command;
use std::path::Path;
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
  match File::open( &Path::new( input_file ) ) {
    Ok( mut x ) => {
      let mut data = vec!();
      x.read_to_end( &mut data ).ok().expect( "Reading file failed" );
      data
    },
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


// Returns exit code
fn printParseTree( grammar_code: &str, input_path: &str ) -> Option<i32> {
  let mut final_code = grammar_code.to_string();
  final_code.push_str( PRINTER_MAIN );
  let temp_dir = TempDir::new( "temp" ).unwrap();
  let code_file = temp_dir.path().join( "printer.rs" );
  let printer = temp_dir.path().join( "printer" );

  match File::create( &code_file ) {
    Ok( mut file ) =>
      file.write_all( final_code.as_bytes() ).ok().expect( "Writing failed" ),
    Err( e ) => panic!( "File error: {}", e ),
  };

  match Command::new( "rustc" ).arg( "-o" )
                               .arg( printer.to_str().unwrap() )
                               .arg( code_file.to_str().unwrap() )
                               .status() {
    Ok( status ) if !status.success() =>
      panic!( "Compiling with rustc failed." ),
    Err( e ) => panic!( "Failed to execute process: {}", e ),
    _ => {}
  };

  let printer = temp_dir.path().join( "printer" );
  let command_output = Command::new(
      printer.to_str().unwrap() ).arg( input_path ).output();

  match command_output {
    Ok( output ) => {
      println!( "{}", String::from_utf8_lossy( &output.stdout ) );
      output.status.code()
    },
    Err( e ) => panic!( "Failed to execute process: {}", e ),
  };

  Some( 0 )
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
        &matches.opt_str( "g" ).unwrap() )[..] )
    .unwrap_or_else( || panic!( "Couldn't parse given PEG grammar" ) )
  } else {
    panic!( "Missing -g option." )
  };


  let exit_code = if matches.opt_present( "i" ) {
    printParseTree( &grammar_code,
                    &matches.opt_str( "i" ).unwrap() )
  } else {
    println!( "{}", grammar_code );
    Some( 0 )
  };

  process::exit( exit_code.unwrap_or( 0 ) );
}

