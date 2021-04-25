#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/leafbuild/leafbuild/master/leaf_icon.svg",
    html_logo_url = "https://raw.githubusercontent.com/leafbuild/leafbuild/master/leaf_icon.svg"
)]
#![forbid(coherence_leak_check, confusable_idents, trivial_bounds)]
#![deny(
    unused_allocation,
    unsafe_code,
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
#![allow(clippy::enum_glob_use)]
#![allow(clippy::wildcard_imports)]

//! The syntax of `build.leaf` files, along with the lexer and parser.
//! This uses [rowan](https://github.com/rust-analyzer/rowan).

///
pub use rowan::TextSize;

use rowan::Language;

pub mod ast;
pub(crate) mod lexer;
pub mod parser;
#[path = "syntax_kind_new.rs"]
pub mod syntax_kind;

use syntax_kind::SyntaxKind;

/// The language
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LeafbuildLanguage;

impl Language for LeafbuildLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        SyntaxKind::from(raw.0)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<u16> for SyntaxKind {
    fn from(i: u16) -> Self {
        assert!(i < Self::__LAST as u16);
        #[allow(unsafe_code)]
        unsafe {
            std::mem::transmute::<u16, Self>(i)
        }
    }
}
impl From<SyntaxKind> for u16 {
    fn from(kind: SyntaxKind) -> Self {
        kind as Self
    }
}
impl From<&SyntaxKind> for u16 {
    fn from(kind: &SyntaxKind) -> Self {
        (*kind).into()
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind.into())
    }
}

impl From<rowan::SyntaxKind> for SyntaxKind {
    fn from(kind: rowan::SyntaxKind) -> Self {
        kind.0.into()
    }
}

#[cfg(test)]
mod tests;
