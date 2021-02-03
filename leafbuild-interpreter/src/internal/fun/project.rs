#[allow(clippy::needless_pass_by_value)]
fn project<'file_frame>(
    file_frame: &'file_frame mut FileFrame,
    args: FnArgsData,
) -> Box<dyn Value<'static>> {
    Box::new(I32Wrap(0))
}

fn project_fn_ty() -> FnTy {
    FnTy {
        positional_params: vec![FnPositionalTy {
            name: String::from("name"),
            ty: Ty::Str,
        }],
        default_params: vec![],
        ret_ty: Box::new(Ty::BuiltinObject(BuiltinTy::Project)),
    }
}

add_builtin_function! {"project", project, project_fn_ty, PROJECT_FUNC, "The `project` function declaration"}
