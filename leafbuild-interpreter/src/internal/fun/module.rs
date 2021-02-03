#[allow(clippy::needless_pass_by_value)]
fn module<'file_frame>(
    file_frame: &'file_frame mut FileFrame,
    args: FnArgsData,
) -> Box<dyn Value<'static>> {
    Box::new(I32Wrap(0))
}

fn module_fn_ty() -> FnTy {
    FnTy {
        positional_params: vec![FnPositionalTy {
            name: String::from("name"),
            ty: Ty::Str,
        }],
        default_params: vec![],
        ret_ty: Box::new(Ty::BuiltinObject(BuiltinTy::Module)),
    }
}

add_builtin_function! {"module", module, module_fn_ty, MODULE_FUNC, "The `module` function declaration"}
