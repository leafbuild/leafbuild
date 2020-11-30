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
use std::num::ParseIntError;

/// Parses the source and returns the definition.
/// # Errors
/// The parse error that prevents the proper AST from being built.
pub fn parse<'input>(
    source: &'input str,
    errors: &mut Vec<ErrorRecovery<usize, lexer::Token<'input>, GrmError>>,
) -> Result<ast::BuildDefinition, ParseError<usize, lexer::Token<'input>, GrmError>> {
    leafparser::BuildDefinitionParser::new()
        .parse(source, errors, lexer::Lexer::new(source))
        .map(ast::BuildDefinition::new)
}

/// A grammar error, happened while parsing.
#[derive(Clone, Debug)]
pub enum GrmError {
    /// A lexical error occurred
    LexError(LexicalError),
    /// Couldn't parse a number from a string somewhere in the source code.
    ParseIntError(ParseIntError, Span),
}

impl GrmError {
    pub(crate) const fn get_span(&self) -> Span {
        match self {
            Self::LexError(ref err) => err.span,
            Self::ParseIntError(_, ref span) => *span,
        }
    }
}

impl From<LexicalError> for GrmError {
    fn from(e: LexicalError) -> Self {
        Self::LexError(e)
    }
}

impl From<(ParseIntError, Span)> for GrmError {
    fn from((e, span): (ParseIntError, Span)) -> Self {
        Self::ParseIntError(e, span)
    }
}
