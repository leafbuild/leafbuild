use crate::diagnostics::{
    DiagnosticsConfig, FileId, LeafDiagnostic, LeafDiagnosticTrait, LeafLabel,
};
use crate::grammar::lexer::{LexicalError, Token};
use lalrpop_util::ParseError;
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
