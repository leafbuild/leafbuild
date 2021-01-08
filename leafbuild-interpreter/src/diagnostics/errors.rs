use itertools::Itertools;
use leafbuild_core::diagnostics::{
    DiagConfig, FileId, LeafDiagnostic, LeafDiagnosticTrait, LeafLabel,
};
use leafbuild_parser::lalrpop_util::ParseError;
use leafbuild_parser::lexer::Token;
use leafbuild_parser::GrmError;
macro_rules! error_codes {
    ($([$name:ident, $file:expr]),* $(,)?) => {
        error_codes!(1, $([$name, $file])*);
    };
    ($start:expr, [$first_name:ident, $first_file:literal] $(,)?) => {
        const $first_name: usize = $start;
        include!(concat!("errors/", $first_file));
    };
    ($start:expr, [$first_name:ident, $first_file:literal], $([$other_name:ident, $other_file:literal]),* $(,)?) => {
        const $first_name: usize = $start;
        include!(concat!("errors/", $first_file));
        error_codes!($start+1, $([$other_name,$other_file])*);
    };
}

error_codes! {[PARSE_ERROR, "parse_error.rs"],}
