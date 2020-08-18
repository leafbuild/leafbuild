pub(crate) struct VariableNotFoundError {
    loc: Location,
}

impl VariableNotFoundError {
    pub(crate) fn new(loc: Location) -> Self {
        Self { loc }
    }
}

impl LeafDiagnosticTrait for VariableNotFoundError {
    fn get_diagnostic(&self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        LeafDiagnostic::error()
            .with_message("Cannot find variable")
            .with_code(VARIABLE_NOT_FOUND_ERROR)
            .with_labels(vec![LeafLabel::primary(ctx.current_file, self.loc.clone())
                .with_message("Variable not found")])
    }

    fn should_print(&self, _: &DiagnosticsCtx) -> bool {
        true
    }
}
