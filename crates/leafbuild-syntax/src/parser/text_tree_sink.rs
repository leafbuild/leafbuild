//! FIXME: docs
use rowan::{GreenNode, GreenNodeBuilder, TextRange, TextSize};

use crate::syntax_kind::SyntaxKind;

use super::error::ParseError;
use super::token_source::Token;
use super::tree_sink::TreeSink;
use super::IsTrivia;

use std::mem;

///
#[derive(Debug)]
pub struct TextTreeSink<'sink> {
    inner: GreenNodeBuilder<'sink>,
    text: &'sink str,
    tokens: &'sink [Token],
    text_pos: TextSize,
    token_pos: usize,
    state: State,
    errors: Vec<(ParseError, TextSize)>,
}
#[derive(Debug)]
enum State {
    PendingStart,
    Normal,
    PendingFinish,
}

impl<'sink> TreeSink for TextTreeSink<'sink> {
    fn token(&mut self, kind: SyntaxKind, n_tokens: u8) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingStart => unreachable!(),
            State::PendingFinish => self.inner.finish_node(),
            State::Normal => (),
        }
        self.eat_trivias();
        let n_tokens = n_tokens as usize;
        let len = self.tokens[self.token_pos..self.token_pos + n_tokens]
            .iter()
            .map(|it| it.len)
            .sum::<TextSize>();
        self.do_token(kind, len, n_tokens);
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingStart => {
                self.inner.start_node(kind.into());
                // No need to attach trivias to previous node: there is no
                // previous node.
                return;
            }
            State::PendingFinish => self.inner.finish_node(),
            State::Normal => (),
        }

        let n_trivias = self.tokens[self.token_pos..]
            .iter()
            .take_while(|it| it.syntax_kind.is_trivia())
            .count();
        let leading_trivias = &self.tokens[self.token_pos..self.token_pos + n_trivias];
        let mut trivia_end =
            self.text_pos + leading_trivias.iter().map(|it| it.len).sum::<TextSize>();

        let n_attached_trivias = {
            let leading_trivias = leading_trivias.iter().rev().map(|it| {
                let next_end = trivia_end - it.len;
                let range = TextRange::new(next_end, trivia_end);
                trivia_end = next_end;
                (it.syntax_kind, &self.text[range])
            });
            n_attached_trivias(kind, leading_trivias)
        };
        self.eat_n_trivias(n_trivias - n_attached_trivias);
        self.inner.start_node(kind.into());
        self.eat_n_trivias(n_attached_trivias);
    }

    fn finish_node(&mut self) {
        match mem::replace(&mut self.state, State::PendingFinish) {
            State::PendingStart => unreachable!(),
            State::PendingFinish => self.inner.finish_node(),
            State::Normal => (),
        }
    }

    fn error(&mut self, error: ParseError) {
        self.errors.push((error, self.text_pos))
    }
}

impl<'sink> TextTreeSink<'sink> {
    pub(super) fn new(text: &'sink str, tokens: &'sink [Token]) -> Self {
        Self {
            text,
            tokens,
            text_pos: 0.into(),
            token_pos: 0,
            state: State::PendingStart,
            inner: GreenNodeBuilder::default(),
            errors: vec![],
        }
    }

    pub(super) fn finish(mut self) -> (GreenNode, Vec<(ParseError, TextSize)>) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingFinish => {
                self.eat_trivias();
                self.inner.finish_node()
            }
            State::PendingStart | State::Normal => unreachable!(),
        }

        (self.inner.finish(), self.errors)
    }

    fn eat_trivias(&mut self) {
        while let Some(&token) = self.tokens.get(self.token_pos) {
            if !token.syntax_kind.is_trivia() {
                break;
            }
            self.do_token(token.syntax_kind, token.len, 1);
        }
    }

    fn eat_n_trivias(&mut self, n: usize) {
        for _ in 0..n {
            let token = self.tokens[self.token_pos];
            assert!(token.syntax_kind.is_trivia());
            self.do_token(token.syntax_kind, token.len, 1);
        }
    }

    fn do_token(&mut self, kind: SyntaxKind, len: TextSize, n_tokens: usize) {
        let range = TextRange::at(self.text_pos, len);
        let text = &self.text[range];
        self.text_pos += len;
        self.token_pos += n_tokens;
        self.inner.token(kind.into(), text);
    }
}

fn n_attached_trivias<'a>(
    kind: SyntaxKind,
    trivias: impl Iterator<Item = (SyntaxKind, &'a str)>,
) -> usize {
    // match kind {
    // TODO: attached trivias
    //     EOF => {
    //         let mut res = 0;
    //         let mut trivias = trivias.enumerate().peekable();

    //         while let Some((i, (kind, text))) = trivias.next() {
    //             match kind {
    //                 WHITESPACE if text.contains("\n\n") => {
    //                     // we check whether the next token is a doc-comment
    //                     // and skip the whitespace in this case
    //                     if let Some((COMMENT, peek_text)) = trivias.peek().map(|(_, pair)| pair) {
    //                         let comment_kind = ast::CommentKind::from_text(peek_text);
    //                         if comment_kind.doc == Some(ast::CommentPlacement::Outer) {
    //                             continue;
    //                         }
    //                     }
    //                     break;
    //                 }
    //                 COMMENT => {
    //                     let comment_kind = ast::CommentKind::from_text(text);
    //                     if comment_kind.doc == Some(ast::CommentPlacement::Inner) {
    //                         break;
    //                     }
    //                     res = i + 1;
    //                 }
    //                 _ => (),
    //             }
    //         }
    //         res
    //     }
    //     _ => 0,
    // }
    0
}
