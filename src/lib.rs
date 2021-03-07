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

//! A work in progress meta build system for C/C++ projects written in Rust.
//!
//! Primary values:
//! - Speed. The Compile part in the Edit-Compile-Run cycle should
//!  take as little as possible.
//!
//! - Compatibility with popular build systems out there.
//!  There is a lot of C/C++ code out there, and your code may
//!  have dependencies that use other build systems. In such
//!  cases, `leafbuild` should try to make sure they will work together.
//!
//! - Keep the syntax as simple as *reasonable*, but don't take the simplicity
//!  part too far(see meson, golang).
//!
//! # Contributions
//! See [CONTRIBUTING.md](https://github.com/leafbuild/leafbuild/blob/master/CONTRIBUTING.md)
//!
//! # MSRV policy
//! This will always work with the latest stable toolchain, though it may compile with older
//! versions.
extern crate codespan_reporting;
#[macro_use]
extern crate tracing;
extern crate leafbuild_derive;
extern crate term;
extern crate thiserror;

pub use cli::run;

#[path = "buildsys-utils/mod.rs"]
pub mod buildsys_utils;
pub mod cli;
pub mod docs;
pub mod trc;
