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
#![allow(unstable)]

use std::str::from_utf8;
use std::char::from_u32;
use std::num::from_str_radix;

// See:
//   http://en.wikipedia.org/wiki/Escape_sequences_in_C
//   http://en.cppreference.com/w/cpp/language/escape
// No question mark escape supported because there are no trigraphs in PEG.
pub fn unescape( input: &[u8] ) -> Vec<u8> {
  let mut final_bytes = Vec::new();
  let mut index = 0;
  loop {
    // TODO: support \u12345678, not just \u1234
    match input.slice_from( index ) {
      [ b'\\', b'u', c1, c2, c3, c4, .. ]
          if isHex( c1 ) && isHex( c2 ) && isHex( c3 ) && isHex( c4 ) => {
        final_bytes = addFourBytesAsCodepoint( final_bytes, [c1, c2, c3, c4] );
        index += 6;
      }
      [ b'\\', b'x', c1, c2, .. ]
          if isHex( c1 ) && isHex( c2 ) => {
        final_bytes = addTwoBytesAsHex( final_bytes, [c1, c2] );
        index += 4;
      }
      [ b'\\', c1, c2, c3, .. ]
          if isOctal( c1 ) && isOctal( c2 ) && isOctal( c3 ) => {
        final_bytes = addThreeBytesAsOctal( final_bytes, [c1, c2, c3] );
        index += 4;
      }
      [ b'\\', c, .. ] => {
        final_bytes = addEscapedByte( final_bytes, c );
        index += 2;
      }
      [ c, .. ] => {
        final_bytes.push( c );
        index += 1;
      }
      [] => break,
    }
  }

  return final_bytes;
}


pub fn unescapeString( input: &str ) -> String {
  from_utf8( unescape( input.as_bytes() ).as_slice() ).unwrap().to_string()
}


fn isOctal( byte: u8 ) -> bool {
  (byte as char).is_digit( 8 )
}


fn isHex( byte: u8 ) -> bool {
  (byte as char).is_digit( 16 )
}


fn addFourBytesAsCodepoint( mut input: Vec<u8>, bytes: [u8; 4] ) -> Vec<u8> {
  let slice = from_utf8( &bytes ).unwrap();
  match from_str_radix( slice, 16 ) {
    Some( x ) => match from_u32( x ) {
      Some( character ) => {
        let mut utf8chars = [0; 4];
        let num_written = character.encode_utf8( &mut utf8chars ).unwrap();
        for i in range( 0, num_written ) {
          input.push( *utf8chars.get( i ).unwrap() );
        }
      },
      _ => panic!( "Invalid unicode code point: {}", x )
    },
    _ => panic!( r"Invalid unicode escape sequence: \u{}{}{}{}",
                bytes.get( 0 ).unwrap(),
                bytes.get( 1 ).unwrap(),
                bytes.get( 2 ).unwrap(),
                bytes.get( 3 ).unwrap() )
  }
  input
}


fn addTwoBytesAsHex( mut input: Vec<u8>, bytes: [u8; 2] ) -> Vec<u8> {
  let slice = from_utf8( &bytes ).unwrap();
  match from_str_radix( slice, 16 ) {
    Some( byte ) => input.push( byte ),
    _ => panic!( r"Invalid hex escape sequence: \x{}{}",
                bytes.get( 0 ).unwrap(),
                bytes.get( 1 ).unwrap() )
  }
  input
}


fn addThreeBytesAsOctal( mut input: Vec<u8>, bytes: [u8; 3] ) -> Vec<u8> {
  let slice = from_utf8( &bytes ).unwrap();
  match from_str_radix( slice, 8 ) {
    Some( byte ) => input.push( byte ),
    _ => panic!( r"Invalid octal escape sequence: \{}{}{}",
                bytes.get( 0 ).unwrap(),
                bytes.get( 1 ).unwrap(),
                bytes.get( 2 ).unwrap() )
  }
  input
}


fn addEscapedByte( mut input: Vec<u8>, byte: u8 ) -> Vec<u8> {
  let unescaped_char = match byte {
    b'a'  => Some( b'\x07' ),
    b'b'  => Some( b'\x08' ),
    b'f'  => Some( b'\x0c' ),
    b'n'  => Some( b'\n'   ),
    b'r'  => Some( b'\r'   ),
    b't'  => Some( b'\t'   ),
    b'v'  => Some( b'\x0b' ),
    b'0'  => Some( b'\0'   ),
    b'\'' => Some( b'\''   ),
    b'"'  => Some( b'"'    ),
    b'\\' => Some( b'\\'   ),
    _     => None
  };

  match unescaped_char {
    Some( x ) => input.push( x ),
    None => {
      input.push( b'\\' );
      input.push( byte )
    }
  };
  input
}


#[cfg(test)]
mod tests {
  use super::{unescape};

  fn vecBytes( input: &'static str ) -> Vec<u8> {
    input.as_bytes().to_vec()
  }

  #[test]
  fn unescape_Nothing() {
    assert_eq!( unescape( b"foobar" ),         vecBytes( "foobar" ) );
    assert_eq!( unescape( b"123"    ),         vecBytes( "123" )    );
    assert_eq!( unescape( "葉".as_bytes() ),   vecBytes( "葉" )     );
    assert_eq!( unescape( "a葉8".as_bytes() ), vecBytes( "a葉8" )   );
  }

  #[test]
  fn unescape_LooksLikeEscapeButNot() {
    assert_eq!( unescape( b"\\z" ), vecBytes( "\\z" ) );
    assert_eq!( unescape( b"a\\zg" ), vecBytes( "a\\zg" ) );
  }

  #[test]
  fn unescape_SingleCharEscapeCodes() {
    assert_eq!( unescape( b"\\a"  ), vecBytes( "\x07" ) );
    assert_eq!( unescape( b"\\b"  ), vecBytes( "\x08" ) );
    assert_eq!( unescape( b"\\f"  ), vecBytes( "\x0c" ) );
    assert_eq!( unescape( b"\\n"  ), vecBytes( "\n" )   );
    assert_eq!( unescape( b"\\r"  ), vecBytes( "\r" )   );
    assert_eq!( unescape( b"\\t"  ), vecBytes( "\t" )   );
    assert_eq!( unescape( b"\\v"  ), vecBytes( "\x0b" ) );
    assert_eq!( unescape( b"\\0"  ), vecBytes( "\0" )   );
    assert_eq!( unescape( b"\\'"  ), vecBytes( "\'" )   );
    assert_eq!( unescape( b"\\\"" ), vecBytes( "\"" )   );
    assert_eq!( unescape( b"\\\\" ), vecBytes( "\\" )   );
  }

  #[test]
  fn unescape_MultipleSingleCharEscapeCodes() {
    assert_eq!( unescape( b"\\r\\n" ), vecBytes( "\r\n" ) );
    assert_eq!( unescape( b"\\\\\\\\" ), vecBytes( "\\\\" ) );
  }

  #[test]
  fn unescape_HexEscape() {
    assert_eq!( unescape( b"\\x4E" ), vecBytes( "N" ) );
    assert_eq!( unescape( b"\\x4e" ), vecBytes( "N" ) );
    assert_eq!( unescape( b"\\x00\\x01" ), vec!( 0, 1 ) );
  }

  #[test]
  fn unescape_HexEscape_Bad() {
    assert_eq!( unescape( b"\\x" ), vecBytes( "\\x" ) );
    assert_eq!( unescape( b"\\xgg" ), vecBytes( "\\xgg" ) );
  }

  #[test]
  fn unescape_UnicodeEscape() {
    assert_eq!( unescape( b"\\u0106" ), vecBytes( "Ć" ) );
    assert_eq!( unescape( b"\\u0106\\u04E8" ), vecBytes( "ĆӨ" ) );
    assert_eq!( unescape( b"\\u81EA\\u7531" ), vecBytes( "自由" ) );
  }

  #[test]
  fn unescape_UnicodeEscape_Bad() {
    assert_eq!( unescape( b"\\u" ), vecBytes( "\\u" ) );
    assert_eq!( unescape( b"\\u01x" ), vecBytes( "\\u01x" ) );
  }

  #[test]
  fn unescape_OctalEscape() {
    assert_eq!( unescape( b"\\001" ), vec!( 1 ) );
    assert_eq!( unescape( b"\\157" ), vec!( 111 ) );
  }
}
