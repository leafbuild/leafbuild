use crate::interpreter::diagnostics::{
    DiagnosticsCtx, LeafDiagnostic, LeafDiagnosticTrait, LeafLabel, Location,
};
macro_rules! warning_codes {
    ($($name:ident),* $(,)?) => {
        warning_codes!(1, $($name,)*);
    };
    ($start:expr, $first:ident $(,)?) => {
        const $first: usize = $start;
    };
    ($start:expr, $first:ident, $($others:ident),* $(,)?) => {
        const $first: usize = $start;
        warning_codes!($start+1, $($others,)*);
    };
}

warning_codes!(VAR_NAME_IN_PRELUDE,);

include!("var_name_in_prelude.rs");
