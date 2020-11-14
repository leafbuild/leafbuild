#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/leafbuild/leafbuild/master/leaf_icon.svg",
    html_logo_url = "https://raw.githubusercontent.com/leafbuild/leafbuild/master/leaf_icon.svg"
)]
#![deny(missing_docs)]
#![deny(missing_crate_level_docs)]
#![deny(unsafe_code)]
#![deny(clippy::correctness)]
#![deny(clippy::style)]
#![deny(clippy::complexity)]
#![deny(clippy::cargo)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

//! A C/C++ buildsystem.
//! # Examples
//! See example usage in the binary.

extern crate codespan_reporting;
#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate log;
extern crate term;
extern crate thiserror;

#[path = "buildsys-utils/mod.rs"]
pub mod buildsys_utils;
pub(crate) mod diagnostics;
pub mod docs;
pub mod grammar;
pub mod handle;
pub mod interpreter;
