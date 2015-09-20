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
#![cfg_attr(test, allow(dead_code))]

use std::str::from_utf8;
use std::char::from_u32;

// See:
//   http://en.wikipedia.org/wiki/Escape_sequences_in_C
//   http://en.cppreference.com/w/cpp/language/escape
// No question mark escape supported because there are no trigraphs in PEG.
pub fn unescape( input: &[u8] ) -> Vec<u8> {
  let mut final_bytes = Vec::new();
  let mut index = 0;

  while index < input.len() {
    if input[ index ] != b'\\' {
      final_bytes.push( input[ index ] );
      index += 1;
      continue;
    }
    let num_chars_past_slash = input.len() - index - 1;

    // TODO: support \u12345678, not just \u1234
    if num_chars_past_slash >= 5 &&
       input[ index + 1 ] == b'u' &&
       isHex( input[ index + 2 ] ) &&
       isHex( input[ index + 3 ] ) &&
       isHex( input[ index + 4 ] ) &&
       isHex( input[ index + 5 ] ) {
      final_bytes = addFourBytesAsCodepoint( final_bytes,
                                             &input[ index + 2 .. index + 6 ] );
      index += 6;
      continue;
    }

    if num_chars_past_slash >= 3 &&
       input[ index + 1 ] == b'x' &&
       isHex( input[ index + 2 ] ) &&
       isHex( input[ index + 3 ] ) {
      final_bytes = addTwoBytesAsHex( final_bytes,
                                      &input[ index + 2 .. index + 4 ] );
      index += 4;
      continue;
    }

    if num_chars_past_slash >= 3 &&
       isOctal( input[ index + 1 ] ) &&
       isOctal( input[ index + 2 ] ) &&
       isOctal( input[ index + 3 ] ) {
      final_bytes = addThreeBytesAsOctal( final_bytes,
                                          &input[ index + 1 .. index + 4 ] );
      index += 4;
      continue;
    }

    if num_chars_past_slash >= 1 {
      final_bytes = addEscapedByte( final_bytes, input[ index + 1 ] );
      index += 2;
    }
  }

  final_bytes
}


pub fn unescapeString( input: &str ) -> String {
  from_utf8( &unescape( input.as_bytes() ) ).unwrap().to_string()
}


fn isOctal( byte: u8 ) -> bool {
  (byte as char).is_digit( 8 )
}


fn isHex( byte: u8 ) -> bool {
  (byte as char).is_digit( 16 )
}


fn addFourBytesAsCodepoint( mut input: Vec<u8>, bytes: &[u8] ) -> Vec<u8> {
  // &[u8] -> &str -> u32 -> char -> encode as utf8 bytes to input

  let slice: &str = from_utf8( &bytes ).unwrap();
  match u32::from_str_radix( slice, 16 ) {
    Ok( x ) => match from_u32( x ) {
      Some( character ) => {
        let mut utf8chars = [0; 4];
        let num_written = encode_utf8_raw( character as u32,
                                           &mut utf8chars ).unwrap();
        for i in 0 .. num_written {
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


fn addTwoBytesAsHex( mut input: Vec<u8>, bytes: &[u8] ) -> Vec<u8> {
  // &[u8] -> &str -> u8 -> write to input

  let slice: &str = from_utf8( &bytes ).unwrap();
  match u8::from_str_radix( slice, 16 ) {
    Ok( byte ) => input.push( byte ),
    _ => panic!( r"Invalid hex escape sequence: \x{}{}",
                bytes.get( 0 ).unwrap(),
                bytes.get( 1 ).unwrap() )
  }
  input
}


fn addThreeBytesAsOctal( mut input: Vec<u8>, bytes: &[u8] ) -> Vec<u8> {
  // &[u8] -> &str -> u8 -> write to input

  let slice: &str = from_utf8( &bytes ).unwrap();
  match u8::from_str_radix( slice, 8 ) {
    Ok( byte ) => input.push( byte ),
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

// UTF-8 ranges and tags for encoding characters
const TAG_CONT: u8    = 0b1000_0000;
const TAG_TWO_B: u8   = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8  = 0b1111_0000;
const MAX_ONE_B: u32   =     0x80;
const MAX_TWO_B: u32   =    0x800;
const MAX_THREE_B: u32 =  0x10000;

// Unstable api, so copied from:
//   https://github.com/rust-lang/rust/blob/master/src/libcore/char.rs#L238
// Replace with upstream char::encode_utf8 when it becomes stable!
#[inline]
pub fn encode_utf8_raw(code: u32, dst: &mut [u8]) -> Option<usize> {
  // Marked #[inline] to allow llvm optimizing it away
  if code < MAX_ONE_B && !dst.is_empty() {
    dst[0] = code as u8;
    Some(1)
  } else if code < MAX_TWO_B && dst.len() >= 2 {
    dst[0] = (code >> 6 & 0x1F) as u8 | TAG_TWO_B;
    dst[1] = (code & 0x3F) as u8 | TAG_CONT;
    Some(2)
  } else if code < MAX_THREE_B && dst.len() >= 3  {
    dst[0] = (code >> 12 & 0x0F) as u8 | TAG_THREE_B;
    dst[1] = (code >>  6 & 0x3F) as u8 | TAG_CONT;
    dst[2] = (code & 0x3F) as u8 | TAG_CONT;
    Some(3)
  } else if dst.len() >= 4 {
    dst[0] = (code >> 18 & 0x07) as u8 | TAG_FOUR_B;
    dst[1] = (code >> 12 & 0x3F) as u8 | TAG_CONT;
    dst[2] = (code >>  6 & 0x3F) as u8 | TAG_CONT;
    dst[3] = (code & 0x3F) as u8 | TAG_CONT;
    Some(4)
  } else {
    None
  }
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
