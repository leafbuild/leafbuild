use crate::interpreter::ModuleData;
use crate::{
    grammar::ast::AstLoc,
    interpreter::{
        diagnostics::{self, errors::*},
        types::TypeId,
        CallExecutor, CallPool, EnvFrameType, Value, ValueTypeMarker,
    },
};
use itertools::Itertools;
use paste::paste;

macro_rules! func_decls {
    ($([$name:ident, $file:expr]),* $(,)?) => {
        $(
            include!($file);
        )*

        paste! {
            pub(crate) fn get_global_functions() -> CallPool {
                CallPool::new(vec![
                    $(
                        [<get_ $name _executor>](),
                    )*
                ])
            }
        }
    };
}

func_decls!(
    [module, "module.rs"],
    [print, "print.rs"],
    [project, "project.rs"],
);
