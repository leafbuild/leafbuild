pub(crate) fn get_global_functions() -> FuncCallPool {
    FuncCallPool::new(vec![
        FuncCallExecutor::new("f".to_string(), |args, frame, _| {
            println!("Called f!");
            Value::new(Box::new(0))
        }),
        FuncCallExecutor::new("print".to_string(), |args, frame, _| {
            println!(
                "-- {}",
                args.get_positional_args()
                    .iter()
                    .map(|arg| { arg.get_value().compute_value_in_env(frame).stringify() })
                    .join(", ")
            );
            Value::new(Box::new(0))
        }),
    ])
}

pub(crate) fn get_func_call_pool_for_typeid(typeid: types::TypeId) -> FuncCallPool {
    match typeid {
        types::TypeId::I32(_)
        | types::TypeId::I64(_)
        | types::TypeId::U32(_)
        | types::TypeId::U64(_) => types::get_num_call_pool(),
        types::TypeId::String(_) => types::get_string_call_pool(),
    }
}
