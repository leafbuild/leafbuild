const MODULE_FUNCTION_DOCS: &str = "reference/functions/project_and_module/module.html";

#[inline]
pub(crate) fn get_module_executor() -> CallExecutor {
    CallExecutor::new(
        "module".to_string(),
        |loc, args, frame, _| {
            let pos_args = args.get_positional_args();
            if pos_args.len() != 1 {
                let loc_clone = loc.clone();
                diagnostics::push_diagnostic(
                    InvalidNumberOfPositionalArguments::new(
                        loc,
                        match pos_args.len() {
                            0 => loc_clone,
                            _ => pos_args.get_rng(),
                        },
                        1,
                        pos_args.len(),
                    )
                    .with_docs_location(MODULE_FUNCTION_DOCS),
                    frame,
                );
                return Value::new(Box::new(()));
            }
            let mod_name_arg = &pos_args[0];
            let mod_name = match mod_name_arg
                .get_value()
                .eval_in_env(frame)
                .get_type_id_and_value()
                .get_string()
            {
                Ok(s) => s.clone(),
                Err(tp) => {
                    diagnostics::push_diagnostic(
                        ExpectedTypeError::new(
                            TypeId::String.typename(),
                            ExprLocAndType::new(mod_name_arg.get_rng(), tp.typename()),
                        )
                        .with_docs_location(MODULE_FUNCTION_DOCS),
                        frame,
                    );
                    return Value::new(Box::new(()));
                }
            };
            frame.fr_type = EnvFrameType::Module(ModuleData { name: mod_name });
            Value::new(Box::new(()))
        },
        vec!["mod".to_string()],
    )
}
