extern crate codespan_reporting;
#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate log;
extern crate paste;
extern crate term;

#[path = "buildsys-utils/mod.rs"]
pub mod buildsys_utils;
pub(crate) mod diagnostics;
pub mod grammar;
pub mod handle;
pub mod interpreter;
