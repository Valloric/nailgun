#!/usr/bin/env python

import codecs
import re
import os.path as p

def StripRules( contents ):
  regex = re.compile( u'// RULES START.*?// RULES END', re.DOTALL )
  return regex.sub( u'', contents )


def StripComments( contents ):
  return re.sub( u'\s*//.*$', u'', contents, flags = re.MULTILINE )


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
    match = re.search( u'(\s*)(pub\s+)?mod (\w+);', contents )
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


def Main():
  parser_contents = FileContents( 'parser.rs' )
  parser_contents = StripRules( parser_contents )
  parser_contents = InlineModules( 'parser.rs', parser_contents )
  parser_contents = StripComments( parser_contents )
  print parser_contents.encode( 'utf-8' )


if __name__ == "__main__":
  Main()
