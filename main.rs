
// TODO: Remove these before releasing!
#[allow(dead_code)]
#[allow(unused_variable)]
#[allow(deprecated_owned_vector)]
mod parser;


#[cfg(not(test))]
fn main() {
  println!( "{:?}", parser::parseBytes( bytes!( "foo" ) ) );
}

