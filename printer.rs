pub static PRINTER_MAIN : &'static str = r###"
fn inputFromFile( input_file: &str ) -> Vec<u8> {
  match std::io::File::open( &Path::new( input_file ) ).read_to_end() {
    Ok( x ) => x,
    _ => fail!( "Couldn't read input file: {}", input_file )
  }
}

fn main() {
  let args = std::os::args();
  match parse( inputFromFile( args.get( 1 ).as_slice() ).as_slice() ) {
    Some( ref node ) => println!( "{}", node ),
    _ => println!( "Couldn't parse input." )
  };
}
"###;
