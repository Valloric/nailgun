#[feature(macro_rules)];

// TODO: Remove these before releasing!
#[allow(dead_code)]
#[allow(unused_variable)]
mod base;


#[cfg(not(test))]
fn main() {
  println!( "{:?}", base::parseBytes( bytes!( "foo" ) ) );
}

