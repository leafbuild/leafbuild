use crate::grammar::GrmError;
use logos::Logos;
use std::fmt;
use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;

/// A span in the source code
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, new)]
pub struct Span {
    start: usize,
    end: usize,
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl Span {
    #[must_use]
    pub(crate) const fn get_rng(&self) -> Range<usize> {
        self.start..self.end
    }

    #[must_use]
    pub(crate) const fn get_start(&self) -> usize {
        self.start
    }

    #[must_use]
    pub(crate) const fn get_end(&self) -> usize {
        self.end
    }
}

impl From<Range<usize>> for Span {
    fn from(r: Range<usize>) -> Self {
        Self {
            start: r.start,
            end: r.end,
        }
    }
}

#[cfg(test)]
#[allow(clippy::fallible_impl_from)]
impl From<Range<i32>> for Span {
    fn from(r: Range<i32>) -> Self {
        use std::convert::TryInto;
        Self {
            start: r.start.try_into().unwrap(),
            end: r.end.try_into().unwrap(),
        }
    }
}

/// The type of the token
#[derive(Logos, Copy, Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum Tk {
    #[token("+=")]
    PlusEq,
    #[token("-=")]
    MinusEq,
    #[token("*=")]
    MulEq,
    #[token("/=")]
    DivEq,
    #[token("%=")]
    ModEq,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Mul,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("==")]
    EqualEqual,
    #[token(">=")]
    GreaterEqual,
    #[token(">")]
    GreaterThan,
    #[token("<=")]
    LessEqual,
    #[token("<")]
    LessThan,
    #[token("!=")]
    NotEqual,
    #[token("=")]
    Equal,
    #[token("<<")]
    ShiftLeft,
    #[token(">>")]
    ShiftRight,
    #[token("(")]
    LParen,
    #[token("[")]
    LBracket,
    #[token("{")]
    LBrace,
    #[token(")")]
    RParen,
    #[token("]")]
    RBracket,
    #[token("}")]
    RBrace,
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token("?")]
    QMark,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token("~")]
    Tilda,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("not")]
    Not,
    #[token("in")]
    In,
    #[token("let")]
    Let,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("foreach")]
    ForEach,
    #[token("continue")]
    Continue,
    #[token("break")]
    Break,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex("([1-9][0-9]*|0x[0-9a-fA-F]+|0b[01]+|0[0-7]+|0)[uU]?[lL]?")]
    Number,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Id,
    #[regex(r#"'(\\['nt\\]|[^'\\])+'"#)]
    String,
    #[regex(r#"'''([^']*|'[^']|''[^'])*'''"#)]
    MultilineString,
    #[regex(r#"//[^\n]*"#)]
    SingleLineComment,
    // #[regex(r#"/\*([^\*]*\*+[^\*/])*([^\*]*\*+|[^\*])*\*/"#)]
    // BlockComment,
    // #[token("\n")]
    // Newline,
    #[error]
    #[regex(r"[ \n\t\r]+", logos::skip)]
    Error,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub struct Token<'data> {
    pub(crate) token: Tk,
    pub(crate) data: &'data str,
}
pub type LxrSpanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[allow(missing_debug_implementations)]
pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, Tk>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(s: &'a str) -> Self {
        let lexer = Tk::lexer(s);
        Self { lexer }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct LexicalError {
    pub(crate) token: Tk,
    pub(crate) span: Span,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LxrSpanned<Token<'a>, usize, GrmError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next().map(|token| match token {
            Tk::Error => Err(GrmError::from(LexicalError {
                token,
                span: Span::from(self.lexer.span()),
            })),
            token => {
                let span = self.lexer.span();
                Ok((
                    span.start,
                    Token {
                        token,
                        data: self.lexer.slice(),
                    },
                    span.end,
                ))
            }
        })
    }
}

/// A number value
#[derive(Copy, Clone, Debug, PartialOrd, Eq, PartialEq)]
pub enum NumVal {
    /// i32 number
    I32(i32),
    /// i64 number
    I64(i64),
    /// u32 number
    U32(u32),
    /// u64 number
    U64(u64),
}

#[derive(Copy, Clone)]
enum Tp {
    I32,
    I64,
    U32,
    U64,
}
impl Tp {
    const fn into_unsigned(self) -> Self {
        match self {
            Self::I32 => Self::U32,
            Self::I64 => Self::U64,
            x => x,
        }
    }

    const fn into_long(self) -> Self {
        match self {
            Self::I32 => Self::I64,
            Self::U32 => Self::U64,
            x => x,
        }
    }

    const fn zero(self) -> NumVal {
        match self {
            Self::I32 => NumVal::I32(0),
            Self::I64 => NumVal::I64(0),
            Self::U32 => NumVal::U32(0),
            Self::U64 => NumVal::U64(0),
        }
    }

    fn create_from_str(self, s: &str, radix: u32) -> Result<NumVal, ParseIntError> {
        match self {
            Self::I32 => i32::from_str_radix(s, radix).map(NumVal::I32),
            Self::U32 => u32::from_str_radix(s, radix).map(NumVal::U32),
            Self::I64 => i64::from_str_radix(s, radix).map(NumVal::I64),
            Self::U64 => u64::from_str_radix(s, radix).map(NumVal::U64),
        }
    }
}

impl FromStr for NumVal {
    type Err = ParseIntError;
    /// parse a number from a number literal string
    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let tp = s
            .chars()
            .rev()
            .take_while(|chr| Self::is_suffix(*chr))
            .fold(Tp::I32, |tp, chr| match chr {
                'u' | 'U' => tp.into_unsigned(),
                'l' | 'L' => tp.into_long(),
                _ => tp,
            });
        if s.starts_with("0x") {
            Self::parse_hex(s, tp)
        } else if s.starts_with("0b") {
            Self::parse_bin(s, tp)
        } else if s.starts_with('0') {
            Self::parse_oct(s, tp)
        } else {
            Self::parse_dec(s, tp)
        }
    }
}

impl NumVal {
    fn parse_hex(s: &str, tp: Tp) -> Result<Self, ParseIntError> {
        // s = "0x.."
        tp.create_from_str(
            s.trim_start_matches("0x").trim_end_matches(Self::is_suffix),
            16,
        )
    }

    fn parse_bin(s: &str, tp: Tp) -> Result<Self, ParseIntError> {
        // s = "0b..."
        tp.create_from_str(
            s.trim_start_matches("0b").trim_end_matches(Self::is_suffix),
            2,
        )
    }

    fn parse_oct(s: &str, tp: Tp) -> Result<Self, ParseIntError> {
        // s = "0..."
        let s = s.trim_start_matches('0').trim_end_matches(Self::is_suffix);
        if s == "" {
            return Ok(tp.zero());
        }

        tp.create_from_str(s, 8)
    }

    fn parse_dec(s: &str, tp: Tp) -> Result<Self, ParseIntError> {
        // s = "..."
        tp.create_from_str(s.trim_end_matches(Self::is_suffix), 10)
    }

    const fn is_suffix(chr: char) -> bool {
        matches!(chr, 'u' | 'U' | 'l' | 'L')
    }
}

#[derive(PartialOrd, PartialEq, Ord, Eq, Debug, Clone)]
pub struct ParsedString(String);

pub fn parse_single_line_string(inp: &str) -> Result<ParsedString, <ParsedString as FromStr>::Err> {
    inp[1..inp.len() - 1].parse()
}

pub fn parse_multi_line_string(inp: &str) -> Result<ParsedString, <ParsedString as FromStr>::Err> {
    inp[3..inp.len() - 3].parse()
}

impl FromStr for ParsedString {
    type Err = snailquote::UnescapeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        snailquote::unescape(s).map(Self)
    }
}

#[cfg(test)]
mod tests;
