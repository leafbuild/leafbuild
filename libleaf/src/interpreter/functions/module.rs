const MODULE_FUNCTION_DOCS: &str = "reference/functions/project_and_module/module.html";

#[inline]
fn get_module_executor() -> CallExecutor {
    CallExecutor::new(
        "module".to_string(),
        |loc, args, frame, _| {
            let pos_args = args.get_positional_args();
            if let Err(e) = require_pos_args(pos_args, loc, frame, 1, MODULE_FUNCTION_DOCS) {
                return e;
            }
            let mod_name_arg = &pos_args[0];
            let mod_name = match mod_name_arg
                .get_value()
                .eval_in_env(frame)
                .get_type_id_and_value_required(TypeId::String)
            {
                Ok(s) => {
                    s.get_string()
                        .unwrap() // guaranteed by the required above that this will be a string.
                        .clone()
                }
                Err(tp) => {
                    diagnostics::push_diagnostic(
                        ExpectedTypeError::new(
                            TypeId::String.typename(),
                            ExprLocAndType::new(mod_name_arg.get_rng(), tp.typename()),
                        )
                        .with_docs_location(MODULE_FUNCTION_DOCS),
                        frame,
                    );
                    return Value::new(Box::new(ErrorValue::new()));
                }
            };
            frame.fr_type = EnvFrameType::Module(ModuleData {
                name: mod_name,
                mod_id: frame.next_mod_id(),
            });
            Value::new(Box::new(()))
        },
        vec!["mod".to_string()],
    )
}
