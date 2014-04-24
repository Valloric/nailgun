#!/usr/bin/env python

import codecs
import re
import subprocess
import os.path as p

DEV_PARSER_FILE = './parser.rs'
PRELUDE_FILE = './prelude.rs'
INLINED_PARSER_FILE = './inlined_parser.rs'
INPUT_PEG_FILE = './examples/original_peg_grammar.peg'


def StripRules( contents ):
  return re.sub( ur'// RULES START.*?// RULES END', u'', contents,
                 flags = re.DOTALL )


def StripComments( contents ):
  return re.sub( ur'\s*(?<!/)//(?!(/|!)).*$',
                 u'',
                 contents,
                 flags = re.MULTILINE )

# We remove the crate_id attribute because the user might want to link several
# different generated parsers; also, our crate_id doesn't make much sense for
# the user's parser.
def StripCrateId( contents ):
  return re.sub( ur'^#!\[crate_id.*?$\n',
                 u'',
                 contents,
                 flags = re.MULTILINE )


def StripTests( contents ):
  while True:
    match = re.search( ur'#\[cfg\(test\)\]\s*?( *)mod tests \{', contents,
                      re.DOTALL | re.MULTILINE )
    if not match:
      break
    mod_indent = ( match.group( 1 ) or u'' ).strip( '\n' )
    regex = re.compile( u'^' + mod_indent +  '\}', re.MULTILINE )
    brace = list( regex.finditer( contents, match.end() ) )[ 0 ]
    contents = contents[ : match.start() ] + contents[ brace.end() : ]
  return contents


def StripExtraWhitespace( contents ):
  # Some previous stages like StripTests can leave several lines of whitespace
  # before '}' chars. We want to remove such whitespace.
  contents = re.sub( ur'\s*?^( *\})', '\n\\1', contents, flags = re.MULTILINE )

  # Also, trailing whitespace is annoying.
  return re.sub( u'[\r \t]+$', '', contents, flags = re.MULTILINE )


def FindModuleFile( module_name, parent_file ):
  dir_start = p.split( parent_file )[ 0 ]
  module_dir = p.join( dir_start, module_name, 'mod.rs' )
  if p.isfile( module_dir ):
    return module_dir
  module_file = p.join( dir_start, module_name + '.rs' )
  if p.isfile( module_file ):
    return module_file
  raise ValueError( 'Module {} not found!'.format( module_name ) )


def FileContents( filename ):
  return codecs.open( filename, 'r', 'utf-8' ).read().strip()


def InlineModules( filename, contents ):
  while True:
    match = re.search( ur'(\s*)(pub\s+)?mod (\w+);', contents )
    if not match:
      break
    module_name = match.group( 3 )
    module_file = FindModuleFile( module_name, filename )
    module_contents = InlineModules( module_file,
                                     FileContents( module_file ) )
    current_indent = match.group( 1 ).strip( '\n' )
    module_contents = re.sub( u'^',
                              current_indent + u'  ',
                              module_contents,
                              flags = re.MULTILINE )
    pub_prefix = match.group( 2 ) or u''
    contents = ( contents[ : match.start( 0 ) ] + (
      u'\n{0}{1}mod {2} {{\n{3}\n{0}}}'.format( current_indent,
                                                pub_prefix,
                                                module_name,
                                                module_contents ) ) +
      contents[ match.end( 0 ) : ] )
  return contents


def PreludeWrap( contents ):
  return ''.join( [
    # We add allow(dead_code) so that the user doesn't get warnings if their
    # generated grammar only uses some features of PEG (and thus only some of
    # the generated code) and not all.
    """pub static PRELUDE : &'static str = r###"#![allow(dead_code)]\n""",
    contents,
    '"###;' ] )


def ExtractRules( inlined_parser ):
  consume = False
  rules = []
  for line in inlined_parser.split( '\n' ):
    if line.startswith( '  rule!( Grammar' ):
      consume = True
    if consume:
      if line.strip():
        rules.append( line )
      else:
        break
  return '\n'.join( rules )


def ReplaceRules( parser, new_rules ):
  match = re.search( ur'// RULES START\n\n(.*?)\s+// RULES END',
                     parser,
                     flags = re.DOTALL | re.MULTILINE )
  return parser[ : match.start( 1 ) ] + new_rules + parser[ match.end( 1 ) : ]


def Main():
  dev_parser = FileContents( DEV_PARSER_FILE )
  prelude = StripRules( dev_parser )
  prelude = InlineModules( DEV_PARSER_FILE, prelude )
  prelude = StripTests( prelude )
  prelude = StripComments( prelude )
  prelude = StripCrateId( prelude )
  prelude = StripExtraWhitespace( prelude )
  prelude = PreludeWrap( prelude )

  with codecs.open( PRELUDE_FILE, 'w+', 'utf-8' ) as f:
    f.write( prelude )

  subprocess.check_output( [ './build', '-c' ] )
  inlined_parser = subprocess.check_output(
    [ './nailed', '-g', INPUT_PEG_FILE ] )

  with codecs.open( INLINED_PARSER_FILE, 'w+', 'utf-8' ) as f:
    f.write( inlined_parser )

  with codecs.open( DEV_PARSER_FILE, 'w+', 'utf-8' ) as f:
    f.write( ReplaceRules( dev_parser, ExtractRules( inlined_parser ) ) )


if __name__ == "__main__":
  Main()
