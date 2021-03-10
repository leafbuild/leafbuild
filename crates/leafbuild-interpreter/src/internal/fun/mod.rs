use crate::env::FileFrame;
use crate::internal::values::Value;
use std::fmt;

/// The `BuiltinFun` declaration
#[derive(Copy, Clone)]
pub struct BuiltinFun {
    name: &'static str,
    fun_handle: for<'file_frame> fn(&'file_frame mut FileFrame) -> Box<dyn Value<'static>>,
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
    ($name:literal, $function_name:expr, $static_name:ident, $documentation:literal) => {
        #[linkme::distributed_slice(crate::internal::fun::BUILTIN_FUNCTIONS)]
        #[used]
        #[no_mangle]
        #[doc = $documentation]
        pub static $static_name: crate::internal::fun::BuiltinFun =
            crate::internal::fun::BuiltinFun {
                name: $name,
                fun_handle: $function_name,
            };
    };
}

pub mod module;
add_builtin_function! {"module", module::module, MODULE_FUNC, "The `module` function declaration"}

pub mod project;
add_builtin_function! {"project", project::project, PROJECT_FUNC, "The `project` function declaration"}
