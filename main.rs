#[feature(macro_rules)];

mod parser;

#[cfg(not(test))]
fn main() {
  println!( "{:?}", base::parseBytes( bytes!( "foo" ) ) );
}

