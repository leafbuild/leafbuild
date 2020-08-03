pub(crate) struct TakeRefError {
    location: Location,
    from: String,
}

impl TakeRefError {
    /// The label would be: `format!("cannot take a reference from {}", from);`
    pub(crate) fn new(location: Location, from: impl Into<String>) -> Self {
        Self {
            location,
            from: from.into(),
        }
    }
}

impl LeafDiagnosticTrait for TakeRefError {
    fn get_diagnostic(self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        LeafDiagnostic::error()
            .with_message("Cannot take a reference from a non-id")
            .with_code(TAKE_REF_ERROR)
            .with_labels(vec![LeafLabel::primary(
                ctx.get_current_file(),
                self.location,
            )])
    }

    fn should_print(&self, _: &DiagnosticsCtx) -> bool {
        true
    }
}
