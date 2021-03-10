//! Contains the [`Lexer`] structure and its internals.
//! It is used as an external lexer to [`lalrpop`](https://github.com/lalrpop/lalrpop).
use crate::GrmError;
use leafbuild_ast::span::Span;
use logos::Logos;

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
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("fn")]
    Fn,
    #[regex("([1-9][0-9]*|0x[0-9a-fA-F]+|0b[01]+|0[0-7]+|0)[uU]?[lL]?")]
    Number,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Id,
    #[regex(r#"'(\\['nt\\]|[^'\\])+'"#)]
    String,
    #[regex(r#"'''([^']*|'[^']|''[^'])*'''"#)]
    MultilineString,
    #[regex(r#"//[^\n]*"#, logos::skip)]
    SingleLineComment,
    #[regex(r"/\*([^*]|\**[^*/])*\*+/", logos::skip)]
    BlockComment,
    // #[token("\n")]
    // Newline,
    #[error]
    #[regex(r"[ \n\t\r]+", logos::skip)]
    // fix for https://github.com/maciejhirsz/logos/issues/180
    #[regex(r"/\*([^*]|\*+[^*/])*\*?")]
    Error,
}

/// The token structure
///
// Lalrpop throws tokens back if it cannot make sense of them, so this is part of the return type
// of [`crate::parse`], and so needs to be public.
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

/// A lexical error.
/// Happens when the lexer wasn't able to understand some input
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

#[cfg(test)]
mod tests;
