//! FIXME: docs
use rowan::TextSize;

use crate::syntax_kind::SyntaxKind;

/// A structure to hold data about a forward token,
/// resulted from a query like "find the next token that is not a newline".
#[derive(Copy, Clone, Debug)]
pub struct ForwardToken {
    /// The token kind.
    pub kind: SyntaxKind,

    /// How many tokens ahead it was found.
    pub offset: usize,

    /// Internal state of the source, such that
    /// if the state modifies and we want to bump
    /// to this token, we know exactly how many tokens
    /// to bump.
    /// For example, the index of the current token
    /// in the stream, or something like that.
    pub(crate) state: usize,
}

impl Default for ForwardToken {
    fn default() -> Self {
        Self {
            kind: SyntaxKind::EOF,
            offset: 0,
            state: 0,
        }
    }
}

/// Describes a query to the token source.
#[derive(Copy, Clone, Debug)]
pub enum FindProperty<'a> {
    /// Query: find the next token such that its kind is in `.0`
    In(&'a [SyntaxKind]),
    /// Query: find the next token such that its kind is not in `.0`
    NotIn(&'a [SyntaxKind]),
    /// Query: find the next token that has the kind `.0`
    KindIs(SyntaxKind),
    /// Query: find the next token that doesn't have the kind `.0`
    KindIsNot(SyntaxKind),
}

/// A source we get the tokens from.
pub trait TokenSource {
    /// Get the current token.
    fn current(&self) -> Token;
    /// Look ahead at the `n`th token.
    fn lookahead(&self, n: usize) -> Token;
    /// Goes on to the next token.
    fn bump(&mut self);

    /// Finds the token with the given property.
    fn find(&self, find_property: FindProperty) -> ForwardToken;

    /// Given a token resulted from a [`find`] query, retunrn how many
    /// bumps of a single token it has to do to get to it.
    fn bump_to(&self, forward_token: ForwardToken) -> usize;
}

/// A token
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Token {
    /// The kind of this token.
    pub syntax_kind: SyntaxKind,
    /// The size of the text in the token.
    pub len: TextSize,
}
