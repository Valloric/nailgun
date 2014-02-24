enum NodeContents {
  Text( ~str ),
  Children( ~[ ~Node ] )
}


struct Node {
  name: ~str,
  start: uint,
  end: uint,
  contents: NodeContents
}


fn parseString( input: &str ) -> Node {
  Node { name: ~"foo",
         start: 0,
         end: 3,
         contents: Text( input.to_owned() ) }
}


fn main() {
  println!( "{:?}", parseString( "foo" ) )
}

