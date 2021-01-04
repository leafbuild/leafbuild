//! General utils and conveniences(mostly taken from kotlin)
//!
//! Note that rust doesn't allow using `self` as the name of
//! a parameter in a closure, so `apply` and `run` are missing,
//! while `with` is kind of redundant
#![allow(clippy::inline_always)]
mod also;
#[path = "let.rs"]
mod let_;

pub use also::*;
pub use let_::*;
