const EXECUTABLE_FUNCTION_DOCS: &str = "reference/functions/executables/executable.html";

fn get_executable_executor() -> CallExecutor {
    CallExecutor::new(
        "executable".to_string(),
        |loc, args, frame, _| {
            let pos_args = args.get_positional_args();
            if let Err(e) = require_pos_args(pos_args, loc, frame, 2, EXECUTABLE_FUNCTION_DOCS) {
                return e;
            }
            let executable_name_arg = &pos_args[0];
            let executable_name = match executable_name_arg
                .get_value()
                .eval_in_env(frame)
                .get_type_id_and_value_required(TypeId::String)
            {
                Ok(v) => v.get_string().unwrap().clone(),
                Err(tp) => {
                    diagnostics::push_diagnostic(
                        ExpectedTypeError::new(
                            TypeId::String.typename(),
                            ExprLocAndType::new(executable_name_arg.get_rng(), tp.typename()),
                        )
                        .with_docs_location(EXECUTABLE_FUNCTION_DOCS),
                        frame,
                    );
                    return Value::new(Box::new(ErrorValue::new()));
                }
            };
            let sources_arg = &pos_args[1];
            let sources = match sources_arg
                .get_value()
                .eval_in_env(frame)
                .get_type_id_and_value()
                .to_vec_of_string(
                    sources_arg.get_rng(),
                    Some(EXECUTABLE_FUNCTION_DOCS),
                    frame.get_diagnostics_ctx(),
                ) {
                Some(vec) => vec,
                None => return Value::new(Box::new(ErrorValue::new())),
            };
            let include_dirs = match args
                .get_named_args()
                .iter()
                .find(|arg| arg.get_name() == "include_dirs")
            {
                Some(v) => match v
                    .get_value()
                    .eval_in_env(frame)
                    .get_type_id_and_value()
                    .to_vec_of_string(
                        v.get_rng(),
                        Some(EXECUTABLE_FUNCTION_DOCS),
                        frame.get_diagnostics_ctx(),
                    ) {
                    Some(vec) => vec,
                    None => return Value::new(Box::new(ErrorValue::new())),
                },
                None => vec![],
            };
            let dependencies = match args
                .get_named_args()
                .iter()
                .find(|arg| arg.get_name() == "dependencies")
            {
                Some(v) => match v
                    .get_value()
                    .eval_in_env(frame)
                    .get_type_id_and_value()
                    .to_vec_of_dependencies(
                        v.get_rng(),
                        Some(EXECUTABLE_FUNCTION_DOCS),
                        frame.get_diagnostics_ctx(),
                    ) {
                    Some(vec) => vec,
                    None => return Value::new(Box::new(ErrorValue::new())),
                },
                None => vec![],
            };
            let exe = frame.new_executable(executable_name, sources, include_dirs, dependencies);
            Value::new(Box::new(exe.make_ref()))
        },
        vec!["bin".to_string(), "binary".to_string()],
    )
}
