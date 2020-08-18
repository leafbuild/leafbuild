const ADD_SUBDIRECTORY_DOCS: &str = "reference/functions/project_and_module/add_subdirectory.html";

fn get_subdir_executor() -> CallExecutor {
    CallExecutor::new(
        "add_subdirectory".to_string(),
        |loc, args, frame, _| {
            let pos_args = args.get_positional_args();
            if let Err(e) = require_pos_args(pos_args, loc, frame, 1, ADD_SUBDIRECTORY_DOCS) {
                return e;
            }
            let subdir_name_arg = &pos_args[0];
            let subdir_name = match subdir_name_arg
                .get_value()
                .eval_in_env(frame)
                .get_type_id_and_value_required(TypeId::String)
            {
                Ok(v) => v.get_string().unwrap().clone(),
                Err(tp) => {
                    diagnostics::push_diagnostic(
                        ExpectedTypeError::new(
                            TypeId::String.typename(),
                            ExprLocAndType::new(subdir_name_arg.get_rng(), tp.typename()),
                        ),
                        frame,
                    );
                    return Value::new(Box::new(ErrorValue::new()));
                }
            };

            let subdir_path = frame.root_path.join(subdir_name);
            start_on_subdir(subdir_path.as_path(), (frame.env_ref, frame.env_mut_ref));
            frame
                .env_mut_ref
                .diagnostics_ctx
                .set_current_file(frame.get_file_id());
            Value::new(Box::new(()))
        },
        vec!["subdir".to_string()],
    )
}
