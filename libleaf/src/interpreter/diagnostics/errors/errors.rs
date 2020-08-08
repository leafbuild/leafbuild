use crate::interpreter::{
    diagnostics::{DiagnosticsCtx, LeafDiagnostic, LeafDiagnosticTrait, LeafLabel, Location},
    types::TypeId,
    DOCS_ROOT,
};

macro_rules! error_codes {
    ($($name:ident),* $(,)?) => {
        error_codes!(1, $($name,)*);
    };
    ($start:expr, $first:ident $(,)?) => {
        const $first: usize = $start;
    };
    ($start:expr, $first:ident, $($others:ident),* $(,)?) => {
        const $first: usize = $start;
        error_codes!($start+1, $($others,)*);
    };
}

error_codes!(
    CANNOT_FIND_CALL_ERROR,
    EXPECTED_TYPE_ERROR,
    INCOMPATIBLE_ASSIGNMENT_ERROR,
    INDEX_OUTSIDE_VECTOR_ERROR,
    INVALID_INDEX_BASE_ERROR,
    INVALID_INDEX_ERROR,
    INVALID_NUMBER_OF_POSITIONAL_ARGUMENTS,
    OPS_TYPE_ERROR,
    OPS_TYPE_ERROR_ERROR,
    SYNTAX_ERROR,
    TAKE_REF_ERROR,
    VARIABLE_NOT_FOUND_ERROR,
);

pub(crate) struct ExprLocAndType {
    loc: Location,
    type_: String,
}

impl ExprLocAndType {
    pub(crate) fn new(loc: Location, type_: impl Into<String>) -> Self {
        Self {
            loc,
            type_: type_.into(),
        }
    }
}

include!("cannot_find_call.rs");
include!("expected_type_error.rs");
include!("incompatible_assignment.rs");
include!("index_outside_vector.rs");
include!("invalid_index.rs");
include!("invalid_index_base.rs");
include!("invalid_number_of_positional_arguments.rs");
include!("ops_type_error.rs");
include!("syntax_error.rs");
include!("take_ref_error.rs");
include!("variable_not_found.rs");
