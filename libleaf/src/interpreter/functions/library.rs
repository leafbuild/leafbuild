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
                .to_vec_of_string(
                    sources_arg.get_rng(),
                    Some(LIBRARY_FUNCTION_DOCS),
                    frame.get_diagnostics_ctx(),
                ) {
                Some(vec) => vec,
                None => return Value::new(Box::new(ErrorValue::new())),
            };
            let internal_include_dirs = match args
                .get_named_args()
                .iter()
                .find(|arg| arg.get_name() == "internal_include_dirs")
            {
                Some(v) => match v
                    .get_value()
                    .eval_in_env(frame)
                    .get_type_id_and_value()
                    .to_vec_of_string(
                        v.get_rng(),
                        Some(LIBRARY_FUNCTION_DOCS),
                        frame.get_diagnostics_ctx(),
                    ) {
                    Some(vec) => vec,
                    None => return Value::new(Box::new(ErrorValue::new())),
                },
                None => vec![],
            };
            let external_include_dirs = match args
                .get_named_args()
                .iter()
                .find(|arg| arg.get_name() == "external_include_dirs")
            {
                Some(v) => match v
                    .get_value()
                    .eval_in_env(frame)
                    .get_type_id_and_value()
                    .to_vec_of_string(
                        v.get_rng(),
                        Some(LIBRARY_FUNCTION_DOCS),
                        frame.get_diagnostics_ctx(),
                    ) {
                    Some(vec) => vec,
                    None => return Value::new(Box::new(ErrorValue::new())),
                },
                None => vec![],
            };
            let language = match args
                .get_named_args()
                .iter()
                .find(|arg| arg.get_name() == "language")
            {
                Some(l) => match l
                    .get_value()
                    .eval_in_env(frame)
                    .get_type_id_and_value_required(TypeId::String)
                {
                    Ok(s) => match s.get_string().unwrap() as &str {
                        "c" | "C" => Some(Language::C),
                        "cpp" | "CPP" | "c++" | "C++" => Some(Language::CPP),
                        _ => None,
                    },
                    Err(tp) => {
                        diagnostics::push_diagnostic(
                            ExpectedTypeError::new(
                                TypeId::String.typename(),
                                ExprLocAndType::new(l.get_value().get_rng(), tp.typename()),
                            ),
                            frame,
                        );
                        None
                    }
                },
                None => None,
            };
            let lib = frame.new_library(
                library_name,
                type_,
                sources,
                internal_include_dirs,
                external_include_dirs,
                language,
            );
            Value::new(Box::new(lib.make_ref()))
        },
        vec!["lib".to_string()],
    )
}