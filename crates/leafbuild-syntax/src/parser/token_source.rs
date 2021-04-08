use crate::syntax_kind::SyntaxKind;

pub trait TokenSource {
    fn current(&self) -> Token;
    fn lookahead(&self, n: usize) -> Token;
    fn bump(&mut self);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Token {
    pub syntax_kind: SyntaxKind,
}
