use logos::Logos;
use std::ops::Range;

#[derive(Copy, Clone, Debug)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub(crate) fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }

    pub(crate) fn as_rng(&self) -> Range<usize> {
        self.start..self.end
    }

    pub(crate) fn get_start(&self) -> usize {
        self.start
    }

    pub(crate) fn get_end(&self) -> usize {
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

#[derive(Logos, Copy, Clone, Debug, PartialEq)]
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
    #[regex("([1-9][0-9]*|0x[0-9a-fA-F]+|0b[01]+|0[0-7]*)[uU]?[lL]?")]
    Number,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Id,
    #[regex(r#"'(\\['nt\\]|[^'\\])+'"#)]
    String,
    #[regex(r#"'''([^']*|'[^']|''[^'])*'''"#)]
    MultilineString,
    #[token("\n")]
    Newline,
    #[error]
    #[regex(r"[ \t\r]+", logos::skip)]
    Error,
}

#[derive(Debug, Clone)]
pub struct Token<'data> {
    pub(crate) token: Tk,
    pub(crate) data: &'data str,
}
pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub(crate) struct Lexer<'a> {
    lexer: logos::Lexer<'a, Tk>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(s: &'a str) -> Self {
        let lexer = Tk::lexer(s);
        Self { lexer }
    }
}

pub struct LexicalError<'data> {
    pub(crate) token: Tk,
    pub(crate) data: &'data str,
    pub(crate) span: Span,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token<'a>, usize, LexicalError<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lexer.next();
        token.map(|token| match token {
            Tk::Error => Err(LexicalError {
                token,
                data: self.lexer.slice(),
                span: Span::from(self.lexer.span()),
            }),
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
