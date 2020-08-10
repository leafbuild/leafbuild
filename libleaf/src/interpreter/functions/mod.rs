use itertools::Itertools;
use paste::paste;

use crate::interpreter::types::{Executable, TypeIdAndValue};
use crate::{
    grammar::{ast::AstLoc, ast::AstPositionalArg},
    interpreter::{
        diagnostics::{self, errors::*, Location},
        types::{ErrorValue, TypeId},
        CallExecutor, CallPool, EnvFrame, EnvFrameType, ModuleData, ProjectData, Value,
        ValueTypeMarker,
    },
};

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

fn require_pos_args(
    pos_args: &[AstPositionalArg],
    loc: Location,
    frame: &EnvFrame,
    required_pos_args: usize,
    doc_location: &'static str,
) -> Result<(), Value<Box<dyn ValueTypeMarker>>> {
    if pos_args.len() != required_pos_args {
        let loc_clone = loc.clone();
        diagnostics::push_diagnostic(
            InvalidNumberOfPositionalArguments::new(
                loc,
                match pos_args.len() {
                    0 => loc_clone,
                    _ => pos_args.get_rng(),
                },
                required_pos_args,
                pos_args.len(),
            )
            .with_docs_location(doc_location),
            frame,
        );
        Err(Value::new(Box::new(ErrorValue::new())))
    } else {
        Ok(())
    }
}

func_decls!(
    [module, "module.rs"],
    [print, "print.rs"],
    [project, "project.rs"],
    [executable, "executable.rs"],
);
