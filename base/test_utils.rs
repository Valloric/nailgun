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
use base::ParseState;

pub fn ToParseState<'a>( bytes: &'a [u8] ) -> ParseState<'a> {
  ParseState { input: bytes, offset: 0 }
}

macro_rules! input_state( ( $ex:expr ) => ( {
      use base::ParseState;
      ParseState { input: $ex.as_bytes(), offset: 0 }
    } ) );
