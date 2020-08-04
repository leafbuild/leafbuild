use crate::interpreter::{
    diagnostics::{DiagnosticsCtx, LeafDiagnostic, LeafDiagnosticTrait, LeafLabel, Location},
    types::TypeId,
};

const CANNOT_FIND_CALL_ERROR: usize = 1;
const INCOMPATIBLE_ASSIGNMENT_ERROR: usize = 2;
const OPS_TYPE_ERROR: usize = 3;
const OPS_TYPE_ERROR_ERROR: usize = 4;
const SYNTAX_ERROR: usize = 5;
const TAKE_REF_ERROR: usize = 6;
const INVALID_INDEX_BASE_ERROR: usize = 7;
const INVALID_INDEX_ERROR: usize = 8;

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
include!("incompatible_assignment.rs");
include!("invalid_index.rs");
include!("invalid_index_base.rs");
include!("ops_type_error.rs");
include!("syntax_error.rs");
include!("take_ref_error.rs");
