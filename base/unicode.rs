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
use std::char;

// More details: http://en.wikipedia.org/wiki/UTF-8#Description
pub static UTF8_1BYTE_FOLLOWING: u8 = 0b11000000;
pub static UTF8_2BYTE_FOLLOWING: u8 = 0b11100000;
pub static UTF8_3BYTE_FOLLOWING: u8 = 0b11110000;

pub fn readCodepoint( input: &[u8] ) -> Option< char > {
  fn isContinuationByte( byte: u8 ) -> bool {
    byte & 0b11000000 == 0b10000000
  }

  fn codepointBitsFromLeadingByte( byte: u8 ) -> u32 {
    let good_bits =
      if isAscii( byte ) {
        byte
      } else if byte & 0b11100000 == UTF8_1BYTE_FOLLOWING {
        byte & 0b00011111
      } else if byte & 0b11110000 == UTF8_2BYTE_FOLLOWING {
        byte & 0b00001111
      } else {
        byte & 0b00000111
      };
    good_bits as u32
  }

  fn codepointBitsFromContinuationByte( byte: u8 ) -> u32 {
    ( byte & 0b00111111 ) as u32
  }

  match input.get( 0 ) {
    Some( first_byte ) => {
      match bytesFollowing( *first_byte ) {
        Some( num_following ) => {
          let mut codepoint: u32 =
            codepointBitsFromLeadingByte( *first_byte ) << 6 * num_following;
          for i in range( 1, num_following + 1 ) {
            match input.get( i ) {
              Some( byte ) if isContinuationByte( *byte ) => {
                codepoint |= codepointBitsFromContinuationByte( *byte ) <<
                  6 * ( num_following - i );
              }
              _ => return None
            }
          }
          char::from_u32( codepoint )
        }
        _ => None
      }
    }
    _ => None
  }
}


pub fn bytesFollowing( byte: u8 ) -> Option< uint > {
  if isAscii( byte ) {
    Some( 0 )
  } else if byte & 0b11100000 == UTF8_1BYTE_FOLLOWING {
    Some( 1 )
  } else if byte & 0b11110000 == UTF8_2BYTE_FOLLOWING {
    Some( 2 )
  } else if byte & 0b11111000 == UTF8_3BYTE_FOLLOWING {
    Some( 3 )
  } else {
    None
  }
}


pub fn isAscii( byte: u8 ) -> bool {
  return byte & 0b10000000 == 0;
}


#[cfg(test)]
mod tests {
  use super::{readCodepoint, UTF8_1BYTE_FOLLOWING};

  #[test]
  fn readCodepoint_Roundtrip_SimpleAscii() {
    assert_eq!( 'a', readCodepoint( b"a" ).unwrap() );
    assert_eq!( 'z', readCodepoint( b"z" ).unwrap() );
    assert_eq!( 'A', readCodepoint( b"A" ).unwrap() );
    assert_eq!( '9', readCodepoint( b"9" ).unwrap() );
    assert_eq!( '*', readCodepoint( b"*" ).unwrap() );
    assert_eq!( '\n', readCodepoint( b"\n" ).unwrap() );
    assert_eq!( '\0', readCodepoint( b"\0" ).unwrap() );
  }


  #[test]
  fn readCodepoint_Roundtrip_NonAscii() {
    // 2 UTF-8 bytes
    assert_eq!( '¢', readCodepoint( "¢".as_bytes() ).unwrap() );

    // 3 UTF-8 bytes
    assert_eq!( '€', readCodepoint( "€".as_bytes() ).unwrap() );

    // 4 UTF-8 bytes
    assert_eq!( '𤭢', readCodepoint( "𤭢".as_bytes() ).unwrap() );

    // Some extras
    assert_eq!( 'Ć', readCodepoint( "Ć".as_bytes() ).unwrap() );
    assert_eq!( 'Ө', readCodepoint( "Ө".as_bytes() ).unwrap() );
    assert_eq!( '自', readCodepoint( "自".as_bytes() ).unwrap() );
    assert_eq!( '由', readCodepoint( "由".as_bytes() ).unwrap() );
  }


  #[test]
  fn readCodepoint_FailsOnBadChars() {
    assert!( readCodepoint( [ 0b11111111 ] ).is_none() );
    assert!( readCodepoint( [ 0b10000000 ] ).is_none() );
    assert!( readCodepoint( [ UTF8_1BYTE_FOLLOWING, 0b11000000 ] ).is_none() );
  }
}
