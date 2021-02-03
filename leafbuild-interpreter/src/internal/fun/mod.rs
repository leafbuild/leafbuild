use crate::env::FileFrame;
use crate::internal::values::types::{FnTy, Ty, FnPositionalTy};
use crate::internal::values::{BuiltinTy, I32Wrap, Value};
use std::collections::BTreeMap;
use std::fmt;

/// The `BuiltinFun` declaration
#[derive(Clone)]
pub struct BuiltinFun {
    name: &'static str,
    fun_handle: FunHandleType,
    fn_ty: fn() -> FnTy,
}

type FunHandleType =
    for<'file_frame> fn(&'file_frame mut FileFrame, FnArgsData) -> Box<dyn Value<'static>>;

struct FnArgsData {
    positional_args: Vec<Box<dyn Value<'static>>>,
    kwargs: BTreeMap<String, Box<dyn Value<'static>>>,
}

impl fmt::Debug for BuiltinFun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BuiltinFun")
            .field("name", &self.name)
            .field("fun_handle", &(&self.fun_handle as *const _))
            .finish()
    }
}

#[linkme::distributed_slice]
pub static BUILTIN_FUNCTIONS: [BuiltinFun] = [..];

macro_rules! add_builtin_function {
    ($name:literal, $function_name:expr, $fn_ty:expr, $static_name:ident, $documentation:literal) => {
        #[linkme::distributed_slice(crate::internal::fun::BUILTIN_FUNCTIONS)]
        #[used]
        #[no_mangle]
        #[doc = $documentation]
        pub static $static_name: crate::internal::fun::BuiltinFun =
            crate::internal::fun::BuiltinFun {
                name: $name,
                fun_handle: $function_name,
                fn_ty: $fn_ty,
            };
    };
}

fn find_builtin_function(function_name: &str) -> Option<&'static BuiltinFun> {
    BUILTIN_FUNCTIONS.iter().find(|it| it.name == function_name)
}

include! {"module.rs"}
include! {"project.rs"}
include! {"executable.rs"}
include! {"library.rs"}
