#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/leafbuild/leafbuild/master/leaf_icon.svg",
    html_logo_url = "https://raw.githubusercontent.com/leafbuild/leafbuild/master/leaf_icon.svg"
)]
#![forbid(
    unsafe_code,
    unused_allocation,
    coherence_leak_check,
    confusable_idents,
    trivial_bounds
)]
#![deny(
    missing_docs,
    missing_crate_level_docs,
    missing_copy_implementations,
    missing_debug_implementations,
    unused_imports,
    unused_import_braces,
    deprecated,
    broken_intra_doc_links,
    where_clauses_object_safety,
    order_dependent_trait_objects,
    unconditional_panic,
    unconditional_recursion,
    indirect_structural_match
)]
#![deny(
    clippy::correctness,
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::nursery
)]
#![allow(clippy::module_name_repetitions)]

//! The parser of the `build.leaf` files.
extern crate leafbuild_ast;

#[macro_use]
pub extern crate lalrpop_util;

use std::num::ParseIntError;

use lalrpop_util::{ErrorRecovery, ParseError};

use leafbuild_ast::ast;

use crate::lexer::LexicalError;
use leafbuild_ast::span::Span;

lalrpop_mod!(
    /// the parser
    #[allow(
        missing_docs,
        missing_crate_level_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        unused_imports,
        clippy::all,
        clippy::correctness,
        clippy::style,
        clippy::complexity,
        clippy::cargo,
        clippy::pedantic,
        clippy::nursery
    )]
    leafparser
);
mod lexer;
pub use lexer::Token;

/// Parses the source and returns the definition.
/// # Errors
/// The parse error that prevents the proper AST from being built.
pub fn parse<'input>(
    source: &'input str,
    errors: &mut Vec<ErrorRecovery<usize, Token<'input>, GrmError>>,
) -> Result<ast::BuildDefinition, ParseError<usize, Token<'input>, GrmError>> {
    leafparser::BuildDefinitionParser::new().parse(source, errors, lexer::Lexer::new(source))
}

/// A grammar error, happened while parsing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GrmError {
    /// A lexical error occurred
    LexError(LexicalError),
    /// Couldn't parse a number from a string somewhere in the source code.
    ParseIntError(ParseIntError, Span),
}

impl GrmError {
    /// Returns the [`Span`] where this error occurred.
    #[must_use]
    pub const fn get_span(&self) -> Span {
        match self {
            Self::LexError(LexicalError { ref span, .. }) | Self::ParseIntError(_, ref span) => {
                *span
            }
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
