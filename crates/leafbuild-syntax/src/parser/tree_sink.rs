use crate::syntax_kind::SyntaxKind;

use super::error::ParseError;
pub trait TreeSink {
    fn token(&mut self, kind: SyntaxKind);
    fn start_node(&mut self, kind: SyntaxKind);
    fn finish_node(&mut self);
    fn error(&mut self, error: ParseError);
}
