// TODO: Remove these before releasing!
#[allow(dead_code)]
#[allow(unused_variable)]
mod parser;


#[cfg(not(test))]
fn main() {
  println!( "{:?}", parser::parseString( "foo" ) )
}

