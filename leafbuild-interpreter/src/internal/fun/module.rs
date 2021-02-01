#[allow(clippy::needless_pass_by_value)]
fn module<'file_frame>(
    file_frame: &'file_frame mut FileFrame,
    args: FnArgsData,
) -> Box<dyn Value<'static>> {
    Box::new(I32Wrap(0))
}

const fn module_ret_ty() -> Ty {
    Ty::BuiltinObject(BuiltinTy::Module)
}

add_builtin_function! {"module", module, module_ret_ty, MODULE_FUNC, "The `module` function declaration"}
