#[allow(clippy::needless_pass_by_value)]
fn project<'file_frame>(
    file_frame: &'file_frame mut FileFrame,
    args: FnArgsData,
) -> Box<dyn Value<'static>> {
    Box::new(I32Wrap(0))
}

const fn project_ret_ty() -> Ty {
    Ty::BuiltinObject(BuiltinTy::Project)
}

add_builtin_function! {"project", project, project_ret_ty, PROJECT_FUNC, "The `project` function declaration"}
