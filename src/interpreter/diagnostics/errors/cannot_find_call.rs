/// An error displayed when we cannot find the function/method to call
pub(crate) struct CannotFindCallError {
    /// the location of the identifier for the function/method name
    call_loc: Location,
    /// the name of the function/method we're trying to call
    call_name: String,
    /// Some if it is a method call on the type TypeId, or None if it is a plain function call.
    base_type: Option<TypeId>,
}

impl CannotFindCallError {
    pub(crate) fn new(
        call_loc: Location,
        call_name: impl Into<String>,
        base_type: Option<TypeId>,
    ) -> Self {
        Self {
            call_loc,
            call_name: call_name.into(),
            base_type,
        }
    }
}

impl LeafDiagnosticTrait for CannotFindCallError {
    fn get_diagnostic(&self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        LeafDiagnostic::error()
            .with_message("Cannot find call")
            .with_code(CANNOT_FIND_CALL_ERROR)
            .with_labels(vec![LeafLabel::primary(
                ctx.current_file,
                self.call_loc.clone(),
            )
            .with_message(match &self.base_type {
                Some(val) => format!(
                    "cannot find method '{}' for type `{}`",
                    self.call_name,
                    val.typename(),
                ),
                None => format!("cannot find function '{}'", self.call_name),
            })])
    }

    fn should_print(&self, _ctx: &DiagnosticsCtx) -> bool {
        true
    }
}
