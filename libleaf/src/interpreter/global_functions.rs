pub(crate) fn get_global_functions() -> CallPool {
    CallPool::new(vec![
        CallExecutor::new("f".to_string(), |_args, _frame, _| {
            println!("Called f!");
            Value::new(Box::new(()))
        }),
        get_print_executor(),
        get_module_executor(),
        get_project_executor(),
    ])
}

#[inline]
fn get_print_executor() -> CallExecutor {
    CallExecutor::new("print".to_string(), |args, frame, _| {
        let named_iter: Vec<String> = args
            .get_named_args()
            .iter()
            .map(|arg| {
                format!(
                    "{}: {}",
                    arg.get_name(),
                    arg.get_value().eval_in_env(frame).stringify()
                )
            })
            .collect();
        println!(
            "-- {}",
            args.get_positional_args()
                .iter()
                .map(|arg| { arg.get_value().eval_in_env(frame).stringify() })
                .chain(named_iter)
                .join(", ")
        );
        Value::new(Box::new(()))
    })
}

#[inline]
fn get_module_executor() -> CallExecutor {
    CallExecutor::new("module".to_string(), |args, frame, _| {
        Value::new(Box::new(()))
    })
}

#[inline]
fn get_project_executor() -> CallExecutor {
    CallExecutor::new("module".to_string(), |args, frame, _| {
        Value::new(Box::new(()))
    })
}
