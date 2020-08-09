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
            {
                TypeIdAndValue::Vec(vec) => vec
                    .iter()
                    .enumerate()
                    .map(
                        |(idx, v)| match v.get_type_id_and_value_required(TypeId::String) {
                            Ok(s) => s.get_string().unwrap().clone(),
                            Err(tp) => {
                                diagnostics::push_diagnostic(
                                    UnexpectedTypeInArray::new(
                                        sources_arg.get_rng(),
                                        tp.typename(),
                                        "string",
                                        idx,
                                    )
                                    .with_docs_location(EXECUTABLE_FUNCTION_DOCS),
                                    frame,
                                );
                                "__error".to_string() // return '__error' and filter those out before collecting
                            }
                        },
                    )
                    .filter(|x| *x != "__error")
                    .collect_vec(),
                TypeIdAndValue::String(s) => vec![s.clone()],
                tp => {
                    diagnostics::push_diagnostic(
                        ExpectedTypeError::new(
                            TypeId::String.typename(),
                            ExprLocAndType::new(sources_arg.get_rng(), tp.degrade().typename()),
                        )
                        .with_docs_location(EXECUTABLE_FUNCTION_DOCS),
                        frame,
                    );
                    return Value::new(Box::new(ErrorValue::new()));
                }
            };
            println!("{}: {:?}", executable_name, sources);
            Value::new(Box::new(()))
        },
        vec!["bin".to_string(), "binary".to_string()],
    )
}
