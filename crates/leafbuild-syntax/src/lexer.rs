use crate::parser::Span;
use crate::syntax_kind::SyntaxKind;
use leafbuild_core::prelude::Let;
use logos::Logos;
use std::convert::TryInto;
use std::ops::Range;

/// The type of the token
#[derive(Logos, Copy, Clone, Debug, PartialEq, PartialOrd, Eq)]
enum Tk {
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
    Tilde,
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
    #[token("\n")]
    Newline,
    #[regex(r"[ \t\r]+")]
    Whitespace,
    #[error]
    // fix for https://github.com/maciejhirsz/logos/issues/180
    #[regex(r"/\*([^*]|\*+[^*/])*\*?")]
    Error,
}

impl From<Tk> for SyntaxKind {
    fn from(tk: Tk) -> Self {
        use SyntaxKind::*;
        match tk {
            Tk::PlusEq => PLUS_EQ,
            Tk::MinusEq => MINUS_EQ,
            Tk::MulEq => MUL_EQ,
            Tk::DivEq => DIV_EQ,
            Tk::ModEq => MOD_EQ,
            Tk::Plus => PLUS,
            Tk::Minus => MINUS,
            Tk::Mul => ASTERISK,
            Tk::Slash => SLASH,
            Tk::Percent => PERCENT,
            Tk::EqualEqual => EQ_EQ,
            Tk::GreaterEqual => GREATER_EQ,
            Tk::GreaterThan => GREATER,
            Tk::LessEqual => LESS_EQ,
            Tk::LessThan => LESS,
            Tk::NotEqual => NOT_EQ,
            Tk::Equal => EQ,
            Tk::ShiftLeft => SHIFT_LEFT,
            Tk::ShiftRight => SHIFT_RIGHT,
            Tk::LParen => L_PAREN,
            Tk::LBracket => L_BRACKET,
            Tk::LBrace => L_BRACE,
            Tk::RParen => R_PAREN,
            Tk::RBracket => R_BRACKET,
            Tk::RBrace => R_BRACE,
            Tk::Dot => DOT,
            Tk::Colon => COLON,
            Tk::QMark => QMARK,
            Tk::Semicolon => SEMICOLON,
            Tk::Comma => COMMA,
            Tk::Tilde => TILDE,
            Tk::And => AND_KW,
            Tk::Or => OR_KW,
            Tk::Not => NOT_KW,
            Tk::In => IN_KW,
            Tk::Let => LET_KW,
            Tk::If => IF_KW,
            Tk::Else => ELSE_KW,
            Tk::ForEach => FOREACH_KW,
            Tk::Continue => CONTINUE_KW,
            Tk::Break => BREAK_KW,
            Tk::Return => RETURN_KW,
            Tk::True => TRUE_KW,
            Tk::False => FALSE_KW,
            Tk::Fn => FN_KW,
            Tk::Number => NUMBER,
            Tk::Id => ID,
            Tk::String => STRING,
            Tk::MultilineString => MULTILINE_STRING,
            Tk::SingleLineComment => LINE_COMMENT,
            Tk::BlockComment => BLOCK_COMMENT,
            Tk::Newline => NEWLINE,
            Tk::Whitespace => WHITESPACE,
            Tk::Error => ERROR,
        }
    }
}

/// The token structure
#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
struct Token<'data> {
    pub(crate) token: Tk,
    pub(crate) data: &'data str,
}

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

impl<'a> Iterator for Lexer<'a> {
    type Item = (SyntaxKind, &'a str, Span);

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next().map(|token| {
            (
                token.into(),
                self.lexer.slice(),
                self.lexer
                    .span()
                    .let_(|it| -> Range<u32> {
                        it.start.try_into().unwrap()..it.end.try_into().unwrap()
                    })
                    .into(),
            )
        })
    }
}

//#[cfg(test)]
//mod tests;
