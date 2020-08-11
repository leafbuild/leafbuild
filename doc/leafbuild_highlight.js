/*
Language: Leaf build definition files
Description: Leaf Build is an open-source system for build automation.
Author: Dinu Blanovschi <dinu.blanovschi@criptext.com>
Website: https://gitlab.com/leafbuild/leafbuild
*/

/** @type LanguageFn */
export default function(hljs) {
  let BACKSLASH_ESCAPE = {
    begin: '\\\\[\\s\\S]', relevance: 0
  };
  let APOS_STRING_MODE = {
    className: 'string',
    begin: '\'',
    end: '\'',
    illegal: '\\n',
    contains: [BACKSLASH_ESCAPE]
  };
  let STRINGS = {
    className: 'string',
    variants: [
      {
        begin: '\'\'\'.*', end: '\'\'\''
      },
      APOS_STRING_MODE
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
    beginKeywords:
        // syntax
        'let in and or not ' +
        // control keywords
        'continue break if else foreach ' +
        // types
        'i32 i64 u32 u64 string error void ' +
        // bool literals
        'true false ' +
        // functions
        'project module executable library print'
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
