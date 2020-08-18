pub(crate) struct SyntaxError {
    location: Location,
    description: String,
}

impl SyntaxError {
    pub(crate) fn new(location: Location, description: impl Into<String>) -> Self {
        Self {
            location,
            description: description.into(),
        }
    }
}

impl LeafDiagnosticTrait for SyntaxError {
    fn get_diagnostic(&self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        LeafDiagnostic::error()
            .with_message("Syntax error")
            .with_code(SYNTAX_ERROR)
            .with_labels(vec![LeafLabel::primary(
                ctx.current_file,
                self.location.clone(),
            )
            .with_message(self.description.clone())])
    }

    fn should_print(&self, _: &DiagnosticsCtx) -> bool {
        true
    }
}
