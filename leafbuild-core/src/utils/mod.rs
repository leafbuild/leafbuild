//! General utils and conveniences
//!
//!
//! `let` and `also` are taken from kotlin.
//! Note that rust doesn't allow using `self` as the name of
//! a parameter in a closure, so `apply` and `run` are missing,
//! while `with` is kind of redundant
//!
//!
#![allow(clippy::inline_always)]
mod also;
#[path = "let.rs"]
mod let_;

pub use also::*;
pub use let_::*;

mod some_none_if;
pub use some_none_if::*;

mod some_none_if_owned;
pub use some_none_if_owned::*;

mod and_then_do;
pub use and_then_do::*;
