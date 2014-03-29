#[feature(macro_rules)];

mod base;

#[cfg(not(test))]
fn main() {
  println!( "{:?}", base::parseBytes( bytes!( "foo" ) ) );
}

