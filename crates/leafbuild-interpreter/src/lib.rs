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

//! Interprets the [`ast`][leafbuild_ast] produced by [`leafbuild_parser`], which it is also
//! responsible of getting.
//!
//! Everything you need to do is invoke `run` on a directory and watch the magic happen, at least
//! until you get the result back.
//! The interpreter module
//! Handles everything related to interpreting the source AST.

#[macro_use]
extern crate tracing;
#[macro_use]
extern crate thiserror;

mod diagnostics;
pub mod env;
pub mod handle;
pub mod interpret;

include!("mod_name.rs");

pub use interpret::execute_on;
