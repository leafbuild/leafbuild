//! The grammar of the `build.leaf` files.
pub mod ast;
lalrpop_mod!(
/// the parser
#[allow(
    missing_docs,
    missing_crate_level_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    clippy::all,
    clippy::correctness, 
    clippy::style,
    clippy::complexity,
    clippy::cargo,
    clippy::pedantic,
    clippy::nursery
)]
pub leafparser, "/grammar/leafparser.rs");
pub(crate) mod lexer;

use crate::grammar::lexer::LexicalError;
use lalrpop_util::{ErrorRecovery, ParseError};
pub use lexer::Span;

/// Parses the source and returns the definition.
/// # Errors
/// The parse error that prevents the proper AST from being built.
pub fn parse<'input>(
    source: &'input str,
    errors: &mut Vec<ErrorRecovery<usize, lexer::Token<'input>, LexicalError>>,
) -> Result<ast::BuildDefinition, ParseError<usize, lexer::Token<'input>, LexicalError>> {
    leafparser::BuildDefinitionParser::new()
        .parse(source, errors, lexer::Lexer::new(source))
        .map(ast::BuildDefinition::new)
}
