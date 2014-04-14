#!/usr/bin/env python

import codecs
import re
import os.path as p

def StripRules( contents ):
  return re.sub( ur'// RULES START.*?// RULES END', u'', contents,
                 flags = re.DOTALL )


def StripComments( contents ):
  return re.sub( ur'\s*//.*$', u'', contents, flags = re.MULTILINE )


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
  return re.sub( ur'\s*?^( *\})', '\n\\1', contents, flags = re.MULTILINE )


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
    """pub static PRELUDE : &'static str = r###"\n""",
    contents,
    '"###;' ] )


def Main():
  input_file = 'parser.rs'
  prelude = FileContents( input_file )
  prelude = StripRules( prelude )
  prelude = InlineModules( input_file, prelude )
  prelude = StripTests( prelude )
  prelude = StripComments( prelude )
  prelude = StripExtraWhitespace( prelude )
  prelude = PreludeWrap( prelude )

  codecs.open( 'prelude.rs', 'w+', 'utf-8' ).write( prelude )

if __name__ == "__main__":
  Main()
