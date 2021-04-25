//! FIXME: docs
use crate::syntax_kind::SyntaxKind;

use super::error::ParseError;
/// A CST builder.
pub trait TreeSink {
    /// Adds a token to the tree, on the current branch, merging together `n_raw_tokens` tokens and with kind `kind`.
    fn token(&mut self, kind: SyntaxKind, n_raw_tokens: u8);
    /// Starts a new node on the current branch, with the [`SyntaxKind`] `kind`.
    fn start_node(&mut self, kind: SyntaxKind);

    /// Finishes the last node it started.
    fn finish_node(&mut self);

    /// Adds an error
    fn error(&mut self, error: ParseError);
}
