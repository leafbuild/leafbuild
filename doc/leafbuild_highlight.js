/*
Language: Leaf build definition files
Description: Leaf Build is an open-source system for build automation.
Author: Dinu Blanovschi <dinu.blanovschi@criptext.com>
Website: https://leafbuild.github.com/leafbuild/
*/

/** @type LanguageFn */
export default function(hljs) {
  let STRINGS = {
    className: 'string',
    variants: [
      {
        begin: '\'\'\'.*', end: '\'\'\''
      },
      {
        begin: '\'[^\'\\n]+', end: '\'',
        illegal: '\\n'
      }
    ]
  };
  let NUMBERS = {
    className: 'number',
    variants: [
      {begin: '[1-9][0-9]*'},
      {begin: '0x[0-9a-fA-F]+'},
      {begin: '0[0-7]*'}
    ],
    relevance: 0
  };
  let KEYWORDS = {
    className: 'keyword',
    beginKeywords: 'project module executable library print'
  };
  return {
    name: 'leafbuild',
    aliases: ['build.leaf'],
    case_insensitive: false,
    contains: [
      KEYWORDS,
      NUMBERS,
      {
        className: 'variable',
        begin: '[A-Za-z_][A-Za-z_0-9]*'
      },
      STRINGS,
      hljs.C_LINE_COMMENT_MODE,
      hljs.C_BLOCK_COMMENT_MODE,
    ],
    // @ts-ignore
    exports: {
      strings: STRINGS,
    }
  };
}
