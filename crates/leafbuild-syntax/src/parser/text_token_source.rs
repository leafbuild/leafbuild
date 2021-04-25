//! FIXME: docs
use leafbuild_stdx::TakeIfUnless;
use rowan::TextSize;

use crate::parser::{IsTrivia, Span};
use crate::syntax_kind::SyntaxKind::EOF;

use super::{FindProperty, ForwardToken, Token, TokenSource};

///
#[derive(Debug)]
pub struct LexerWrap {
    pos: usize,
    tokens: Vec<(Token, TextSize)>,
}

impl LexerWrap {
    /// Creates a new `LexerWrap` from the list of tokens and starts at token indexed `0`
    pub fn new(tokens: &[Token]) -> Self {
        let mut text_off = 0.into();
        let tokens = tokens
            .iter()
            .copied()
            .filter_map(|it| {
                let tk = (it, text_off);
                text_off += it.len;
                tk.take_unless(|tk| tk.0.syntax_kind.is_trivia())
            })
            .collect();
        Self { pos: 0, tokens }
    }
}

impl TokenSource for LexerWrap {
    fn current(&self) -> Token {
        self.lookahead(0)
    }

    fn lookahead(&self, n: usize) -> Token {
        self.tokens
            .get(self.pos + n)
            .copied()
            .map(|it| it.0)
            .unwrap_or(Token {
                syntax_kind: EOF,
                len: Span::from(0..0).text_range.len(),
            })
    }

    fn bump(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn find(&self, find_property: FindProperty) -> ForwardToken {
        const NOT_FOUND: ForwardToken = ForwardToken {
            kind: EOF,
            offset: 0,
            state: 0,
        };
        if self.tokens.len() <= self.pos {
            return NOT_FOUND;
        }

        match find_property {
            FindProperty::In(kinds) => self.tokens[self.pos..]
                .iter()
                .enumerate()
                .find(|&(_, &(token, _))| kinds.iter().any(|it| *it == token.syntax_kind))
                .map_or(NOT_FOUND, |it| ForwardToken {
                    kind: it.1 .0.syntax_kind,
                    offset: it.0,
                    state: self.pos,
                }),
            FindProperty::NotIn(kinds) => self.tokens[self.pos..]
                .iter()
                .enumerate()
                .find(|&(_, &(token, _))| !kinds.iter().any(|it| *it == token.syntax_kind))
                .map_or(NOT_FOUND, |it| ForwardToken {
                    kind: it.1 .0.syntax_kind,
                    offset: it.0,
                    state: self.pos,
                }),
            FindProperty::KindIs(kind) => self.tokens[self.pos..]
                .iter()
                .enumerate()
                .find(|&(_, &(token, _))| kind == token.syntax_kind)
                .map_or(NOT_FOUND, |it| ForwardToken {
                    kind: it.1 .0.syntax_kind,
                    offset: it.0,
                    state: self.pos,
                }),
            FindProperty::KindIsNot(kind) => self.tokens[self.pos..]
                .iter()
                .enumerate()
                .find(|&(_, &(token, _))| kind != token.syntax_kind)
                .map_or(NOT_FOUND, |it| ForwardToken {
                    kind: it.1 .0.syntax_kind,
                    offset: it.0,
                    state: self.pos,
                }),
        }
    }

    fn bump_to(&self, forward_token: ForwardToken) -> usize {
        if forward_token.kind == EOF {
            return 0;
        }

        forward_token.state + forward_token.offset - self.pos
    }
}
