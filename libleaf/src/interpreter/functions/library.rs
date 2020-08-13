const LIBRARY_FUNCTION_DOCS: &str = "reference/functions/libraries/library.html";

fn get_library_executor() -> CallExecutor {
    CallExecutor::new(
        "library".to_string(),
        |loc, args, frame, _| {
            let pos_args = args.get_positional_args();
            if let Err(e) = require_pos_args(pos_args, loc, frame, 3, LIBRARY_FUNCTION_DOCS) {
                return e;
            }
            let library_name_arg = &pos_args[0];
            let library_name = match library_name_arg
                .get_value()
                .eval_in_env(frame)
                .get_type_id_and_value_required(TypeId::String)
            {
                Ok(v) => v.get_string().unwrap().clone(),
                Err(tp) => {
                    diagnostics::push_diagnostic(
                        ExpectedTypeError::new(
                            TypeId::String.typename(),
                            ExprLocAndType::new(library_name_arg.get_rng(), tp.typename()),
                        )
                        .with_docs_location(LIBRARY_FUNCTION_DOCS),
                        frame,
                    );
                    return Value::new(Box::new(ErrorValue::new()));
                }
            };
            let type_arg = &pos_args[1];
            let type_ = match type_arg
                .get_value()
                .eval_in_env(frame)
                .get_type_id_and_value_required(TypeId::LibType)
            {
                Ok(v) => v.get_lib_type().unwrap(),
                Err(tp) => {
                    diagnostics::push_diagnostic(
                        ExpectedTypeError::new(
                            TypeId::LibType.typename(),
                            ExprLocAndType::new(type_arg.get_rng(), tp.typename()),
                        )
                        .with_docs_location(LIBRARY_FUNCTION_DOCS),
                        frame,
                    );
                    return Value::new(Box::new(ErrorValue::new()));
                }
            };
            let sources_arg = &pos_args[2];
            let sources = match sources_arg
                .get_value()
                .eval_in_env(frame)
                .get_type_id_and_value()
            {
                TypeIdAndValue::Vec(vec) => vec
                    .iter()
                    .enumerate()
                    .filter_map(
                        |(idx, v)| match v.get_type_id_and_value_required(TypeId::String) {
                            Ok(s) => Some(s.get_string().unwrap().clone()),
                            Err(tp) => {
                                diagnostics::push_diagnostic(
                                    UnexpectedTypeInArray::new(
                                        sources_arg.get_rng(),
                                        tp.typename(),
                                        TypeId::String.typename(),
                                        idx,
                                    )
                                    .with_docs_location(LIBRARY_FUNCTION_DOCS),
                                    frame,
                                );
                                None
                            }
                        },
                    )
                    .collect_vec(),
                TypeIdAndValue::String(s) => vec![s.clone()],
                tp => {
                    diagnostics::push_diagnostic(
                        ExpectedTypeError::new(
                            TypeId::Vec.typename(),
                            ExprLocAndType::new(sources_arg.get_rng(), tp.degrade().typename()),
                        )
                        .with_docs_location(LIBRARY_FUNCTION_DOCS),
                        frame,
                    );
                    return Value::new(Box::new(ErrorValue::new()));
                }
            };
            let internal_include_dirs = match args
                .get_named_args()
                .iter()
                .find(|arg| arg.get_name() == "internal_include_dirs")
            {
                Some(v) => match v.get_value().eval_in_env(frame).get_type_id_and_value() {
                    TypeIdAndValue::Vec(vec) => vec
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, val)| {
                            match val.get_type_id_and_value_required(TypeId::String) {
                                Ok(s) => Some(s.get_string().unwrap().clone()),
                                Err(tp) => {
                                    diagnostics::push_diagnostic(
                                        UnexpectedTypeInArray::new(
                                            v.get_rng(),
                                            tp.typename(),
                                            TypeId::String.typename(),
                                            idx,
                                        )
                                        .with_docs_location(LIBRARY_FUNCTION_DOCS),
                                        frame,
                                    );
                                    None
                                }
                            }
                        })
                        .collect_vec(),
                    TypeIdAndValue::String(s) => vec![s.clone()],
                    tp => {
                        diagnostics::push_diagnostic(
                            ExpectedTypeError::new(
                                TypeId::String.typename(),
                                ExprLocAndType::new(v.get_rng(), tp.degrade().typename()),
                            )
                            .with_docs_location(LIBRARY_FUNCTION_DOCS),
                            frame,
                        );
                        return Value::new(Box::new(ErrorValue::new()));
                    }
                },
                None => vec![],
            };
            let lib = frame.new_library(library_name, type_, sources, internal_include_dirs);
            Value::new(Box::new(lib.make_ref()))
        },
        vec!["lib".to_string()],
    )
}
