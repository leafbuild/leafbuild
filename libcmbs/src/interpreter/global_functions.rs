pub(crate) fn get_global_functions() -> FuncCallPool {
    FuncCallPool::new(vec![
        FuncCallExecutor::new("f".to_string(), |args, frame| {
            println!("Called f!");
            Value::new(Box::new(0))
        }),
        FuncCallExecutor::new("print".to_string(), |args, frame| {
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
