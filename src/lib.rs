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

//! A C/C++ buildsystem.
//! # Examples
//! See example usage in the binary.

extern crate codespan_reporting;
#[macro_use]
extern crate tracing;
extern crate derive_new;
extern crate leafbuild_derive;
extern crate term;
extern crate thiserror;

pub use cli::run;

#[path = "buildsys-utils/mod.rs"]
pub mod buildsys_utils;
pub mod cli;
pub mod docs;
pub mod trc;
