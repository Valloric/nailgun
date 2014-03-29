use std::u8;
use std::char::from_u32;
use std::char;
use std::u32;

// TODO: Fix this when the following issue is fixed:
//   https://github.com/mozilla/rust/issues/4334
static X_U8: u8 = 'x' as u8;
static A_U8: u8 = 'a' as u8;
static B_U8: u8 = 'b' as u8;
static F_U8: u8 = 'f' as u8;
static N_U8: u8 = 'n' as u8;
static R_U8: u8 = 'r' as u8;
static T_U8: u8 = 't' as u8;
static U_U8: u8 = 'u' as u8;
static V_U8: u8 = 'v' as u8;
static ZERO_U8: u8 = '0' as u8;
static SINGLE_QUOTE_U8: u8 = '\'' as u8;
static DOUBLE_QUOTE_U8: u8 = '"' as u8;
static SLASH_U8: u8 = '\\' as u8;


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
      [ SLASH_U8, U_U8, c1, c2, c3, c4, .. ]
          if isHex( c1 ) && isHex( c2 ) && isHex( c3 ) && isHex( c4 ) => {
        final_bytes = addFourBytesAsCodepoint( final_bytes, [c1, c2, c3, c4] );
        index += 6;
      }
      [ SLASH_U8, X_U8, c1, c2, .. ]
          if isHex( c1 ) && isHex( c2 ) => {
        final_bytes = addTwoBytesAsHex( final_bytes, [c1, c2] );
        index += 4;
      }
      [ SLASH_U8, c1, c2, c3, .. ]
          if isOctal( c1 ) && isOctal( c2 ) && isOctal( c3 ) => {
        final_bytes = addThreeBytesAsOctal( final_bytes, [c1, c2, c3] );
        index += 4;
      }
      [ SLASH_U8, c, .. ] => {
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

// TODO: move to somewhere more generic
pub fn vecBytes( input: &'static str ) -> Vec<u8> {
  Vec::from_slice( input.as_bytes() )
}


fn isOctal( byte: u8 ) -> bool {
  char::is_digit_radix( byte as char, 8 )
}


fn isHex( byte: u8 ) -> bool {
  char::is_digit_radix( byte as char, 16 )
}


fn addFourBytesAsCodepoint( mut input: Vec<u8>, bytes: [u8, ..4] ) -> Vec<u8> {
  match u32::parse_bytes( bytes, 16 ) {
    Some( x ) => match from_u32( x ) {
      Some( character ) => {
        let utf8chars: &mut [u8] = [0, ..4];
        let num_written = character.encode_utf8( utf8chars );
        for i in range( 0, num_written ) {
          input.push( *utf8chars.get( i ).unwrap() );
        }
      },
      _ => fail!( "Invalid unicode code point: {}", x )
    },
    _ => fail!( "Invalid unicode escape sequence: \\\\u{}{}{}{}",
                bytes.get( 0 ).unwrap(),
                bytes.get( 1 ).unwrap(),
                bytes.get( 2 ).unwrap(),
                bytes.get( 3 ).unwrap() )
  }
  input
}


fn addTwoBytesAsHex( mut input: Vec<u8>, bytes: [u8, ..2] ) -> Vec<u8> {
  match u8::parse_bytes( bytes, 16 ) {
    Some( byte ) => input.push( byte ),
    _ => fail!( "Invalid hex escape sequence: \\\\x{}{}",
                bytes.get( 0 ).unwrap(),
                bytes.get( 1 ).unwrap() )
  }
  input
}


fn addThreeBytesAsOctal( mut input: Vec<u8>, bytes: [u8, ..3] ) -> Vec<u8> {
  match u8::parse_bytes( bytes, 8 ) {
    Some( byte ) => input.push( byte ),
    _ => fail!( "Invalid octal escape sequence: \\\\{}{}{}",
                bytes.get( 0 ).unwrap(),
                bytes.get( 1 ).unwrap(),
                bytes.get( 2 ).unwrap() )
  }
  input
}



fn addEscapedByte( mut input: Vec<u8>, byte: u8 ) -> Vec<u8> {
  let unescaped_char = match byte {
    A_U8            => Some( '\x07' as u8 ),
    B_U8            => Some( '\x08' as u8 ),
    F_U8            => Some( '\x0c' as u8 ),
    N_U8            => Some( '\n' as u8   ),
    R_U8            => Some( '\r' as u8   ),
    T_U8            => Some( '\t' as u8   ),
    V_U8            => Some( '\x0b' as u8 ),
    ZERO_U8         => Some( '\0' as u8   ),
    SINGLE_QUOTE_U8 => Some( '\'' as u8   ),
    DOUBLE_QUOTE_U8 => Some( '"' as u8    ),
    SLASH_U8        => Some( '\\' as u8   ),
    _               => None
  };

  match unescaped_char {
    Some( x ) => input.push( x ),
    None => {
      input.push( '\\' as u8 );
      input.push( byte )
    }
  };
  input
}


#[cfg(test)]
mod tests {
  use super::{unescape, vecBytes};

  #[test]
  fn unescape_Nothing() {
    assert_eq!( unescape( bytes!( "foobar" ) ), vecBytes( "foobar" ) );
    assert_eq!( unescape( bytes!( "123" )    ), vecBytes( "123" )    );
    assert_eq!( unescape( bytes!( "葉" )     ), vecBytes( "葉" )     );
    assert_eq!( unescape( bytes!( "a葉8" )   ), vecBytes( "a葉8" )   );
  }

  #[test]
  fn unescape_LooksLikeEscapeButNot() {
    assert_eq!( unescape( bytes!( "\\z" ) ), vecBytes( "\\z" ) );
    assert_eq!( unescape( bytes!( "a\\zg" ) ), vecBytes( "a\\zg" ) );
  }

  #[test]
  fn unescape_SingleCharEscapeCodes() {
    assert_eq!( unescape( bytes!( "\\a" )  ), vecBytes( "\x07" ) );
    assert_eq!( unescape( bytes!( "\\b" )  ), vecBytes( "\x08" ) );
    assert_eq!( unescape( bytes!( "\\f" )  ), vecBytes( "\x0c" ) );
    assert_eq!( unescape( bytes!( "\\n" )  ), vecBytes( "\n" )   );
    assert_eq!( unescape( bytes!( "\\r" )  ), vecBytes( "\r" )   );
    assert_eq!( unescape( bytes!( "\\t" )  ), vecBytes( "\t" )   );
    assert_eq!( unescape( bytes!( "\\v" )  ), vecBytes( "\x0b" ) );
    assert_eq!( unescape( bytes!( "\\0" )  ), vecBytes( "\0" )   );
    assert_eq!( unescape( bytes!( "\\'" )  ), vecBytes( "\'" )   );
    assert_eq!( unescape( bytes!( "\\\"" ) ), vecBytes( "\"" )   );
    assert_eq!( unescape( bytes!( "\\\\" ) ), vecBytes( "\\" )   );
  }

  #[test]
  fn unescape_MultipleSingleCharEscapeCodes() {
    assert_eq!( unescape( bytes!( "\\r\\n" ) ), vecBytes( "\r\n" ) );
    assert_eq!( unescape( bytes!( "\\\\\\\\" ) ), vecBytes( "\\\\" ) );
  }

  #[test]
  fn unescape_HexEscape() {
    assert_eq!( unescape( bytes!( "\\x4E" ) ), vecBytes( "N" ) );
    assert_eq!( unescape( bytes!( "\\x4e" ) ), vecBytes( "N" ) );
    assert_eq!( unescape( bytes!( "\\x00\\x01" ) ), vec!( 0, 1 ) );
  }

  #[test]
  fn unescape_HexEscape_Bad() {
    assert_eq!( unescape( bytes!( "\\x" ) ), vecBytes( "\\x" ) );
    assert_eq!( unescape( bytes!( "\\xgg" ) ), vecBytes( "\\xgg" ) );
  }

  #[test]
  fn unescape_UnicodeEscape() {
    assert_eq!( unescape( bytes!( "\\u0106" ) ), vecBytes( "Ć" ) );
    assert_eq!( unescape( bytes!( "\\u0106\\u04E8" ) ), vecBytes( "ĆӨ" ) );
    assert_eq!( unescape( bytes!( "\\u81EA\\u7531" ) ), vecBytes( "自由" ) );
  }

  #[test]
  fn unescape_UnicodeEscape_Bad() {
    assert_eq!( unescape( bytes!( "\\u" ) ), vecBytes( "\\u" ) );
    assert_eq!( unescape( bytes!( "\\u01x" ) ), vecBytes( "\\u01x" ) );
  }

  #[test]
  fn unescape_OctalEscape() {
    assert_eq!( unescape( bytes!( "\\001" ) ), vec!( 1 ) );
    assert_eq!( unescape( bytes!( "\\157" ) ), vec!( 111 ) );
  }
}
