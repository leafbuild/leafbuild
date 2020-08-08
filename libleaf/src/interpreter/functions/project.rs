#[inline]
pub(crate) fn get_project_executor() -> CallExecutor {
    CallExecutor::new(
        "project".to_string(),
        |_loc, args, frame, _| {
            // frame.fr_type = EnvFrameType::Project;
            Value::new(Box::new(()))
        },
        vec![],
    )
}
