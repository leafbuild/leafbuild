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

include!("cannot_find_call.rs");
include!("incompatible_assignment.rs");
include!("ops_type_error.rs");
include!("syntax_error.rs");
include!("take_ref_error.rs");
