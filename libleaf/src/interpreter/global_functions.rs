pub(crate) fn get_global_functions() -> CallPool {
    CallPool::new(vec![
        CallExecutor::new("f".to_string(), |_args, _frame, _| {
            println!("Called f!");
            Value::new(Box::new(0))
        }),
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
            Value::new(Box::new(0))
        }),
    ])
}

pub(crate) fn get_func_call_pool_for_typeid(typeid: types::TypeId) -> CallPool {
    match typeid {
        types::TypeId::I32 | types::TypeId::I64 | types::TypeId::U32 | types::TypeId::U64 => {
            types::get_num_call_pool()
        }
        types::TypeId::String => types::get_string_call_pool(),
    }
}
