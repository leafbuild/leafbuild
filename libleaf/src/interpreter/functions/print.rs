#[inline]
pub(crate) fn get_print_executor() -> CallExecutor {
    CallExecutor::new(
        "print".to_string(),
        |_loc, args, frame, _| {
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
        },
        vec![],
    )
}
