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
pub static PRINTER_MAIN : &'static str = r###"
fn inputFromFile( input_file: &str ) -> Vec<u8> {
  match std::io::File::open( &Path::new( input_file ) ).read_to_end() {
    Ok( x ) => x,
    _ => panic!( "Couldn't read input file: {}", input_file )
  }
}

fn main() {
  let args = std::os::args();
  match parse( inputFromFile( args.get( 1 )[] )[] ) {
    Some( ref node ) => println!( "{}", node ),
    _ => {
      println!( "Couldn't parse input." );
      std::os::set_exit_status( 1 );
    }
  };
}
"###;
