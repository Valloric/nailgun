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
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![cfg_attr(test, allow(dead_code))]
#![deny(deprecated)]

extern crate getopts;
extern crate tempdir;
extern crate inlined_parser;

use tempdir::TempDir;
#[cfg(not(test))]
use getopts::Options;
use std::env;
use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;
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

#[derive(Debug)]
enum CliError {
  Io( io::Error ),
  Misc( String ),
}


impl From<io::Error> for CliError {
  fn from( error: io::Error ) -> CliError {
    CliError::Io( error )
  }
}


impl From<String> for CliError {
  fn from( error: String ) -> CliError {
    CliError::Misc( error )
  }
}


fn inputFromFile( input_file: &str ) -> Result<Vec<u8>, CliError> {
  File::open( &Path::new( input_file ) ).and_then( |mut file| {
    let mut data: Vec<u8> = vec!();
    file.read_to_end( &mut data ).map( |_| data )
  }).map_err( |_| CliError::Misc( format!( "Couldn't read: {}", input_file ) ) )
}


fn indentLines( input: &str, num_spaces: usize ) -> String {
  let indent: String = repeat( " " ).take( num_spaces ).collect();
  input.split( '\n' ).map(
    |x| indent.clone() + x + "\n" ).collect::<Vec<String>>().concat()
}


fn printUsage( opts: &getopts::Options ) {
  let program_path = env::args().next().unwrap();
  let program = Path::new( &program_path );
  let short = opts.short_usage(
    program.file_name().unwrap().to_str().unwrap() );
  let usage = opts.usage( &short );
  println!( "{}", usage );
}


fn nameOfFirstRule<'a>( root: &'a Node<'a> ) -> String {
  str::from_utf8(
    &root.preOrder().find( |x| x.name == "Identifier" ).unwrap()
        .matchedData() ).unwrap().trim_matches(' ').to_string()
}


fn codeForGrammar( input: &[u8] ) -> Result<String, CliError> {
  parse( input ).map( |node| {
    let parse_rules = indentLines( &generator::codeForNode( &node ), 2 );
    let prepared_prelude = PRELUDE[ .. PRELUDE.len() -1 ].replace(
      TOP_LEVEL_RULE,
      &nameOfFirstRule( &node ) );

    prepared_prelude + "\n" + &parse_rules + "}"
  } ).ok_or( CliError::Misc( "Failed to parse PEG grammar".to_string() ) )
}


// Returns exit code
fn printParseTree( grammar_code: &str, input_path: &str )
     -> Result<i32, CliError> {
  let final_code = grammar_code.to_owned() + PRINTER_MAIN;
  let temp_dir = TempDir::new( "temp" ).unwrap();
  let code_file = temp_dir.path().join( "printer.rs" );
  let printer = temp_dir.path().join( "printer" );

  try!( File::create( &code_file ).and_then( |mut file| {
    file.write_all( final_code.as_bytes() )
  } ) );

  let status = try!( Command::new( "rustc" ).arg( "-o" )
                     .arg( printer.to_str().unwrap() )
                     .arg( code_file.to_str().unwrap() )
                     .status() );

  if !status.success() {
    return Err( CliError::Misc( "Failed to write code file".to_string() ) )
  }

  let printer = temp_dir.path().join( "printer" );
  let output = try!( Command::new( printer.to_str().unwrap() )
                     .arg( input_path ).output() );

  println!( "{}", String::from_utf8_lossy( &output.stdout ) );
  output.status.code().ok_or(
    CliError::Misc( "No status code for process.".to_string() ) )
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
  let matches = opts.parse( &args[ 1.. ] ).unwrap();
  if matches.opt_present( "h" ) || args.len() < 2 {
    printUsage( &opts );
    return;
  }

  let exit_code = matches.opt_str( "g" )
    .ok_or( CliError::Misc( "Missing -g option".to_string() ) )
    .and_then( |file| inputFromFile( &file ) )
    .and_then( |input| codeForGrammar( &input ) )
    .and_then( |grammar_code| {
      if let Some( input_path ) = matches.opt_str( "i" ) {
        printParseTree( &grammar_code, &input_path )
      } else {
        println!( "{}", grammar_code );
        Ok( 0 )
      }
    } );

  std::process::exit( exit_code.unwrap_or_else( |error| {
    println!( "{:?}", error );
    1
  } ) );
}

