pub(crate) struct LibValidationError {
    message: String,
    location: Location,
}

impl LibValidationError {
    pub(crate) fn new(message: String, location: Location) -> Self {
        Self { message, location }
    }
}

impl LeafDiagnosticTrait for LibValidationError {
    fn get_diagnostic(&self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        LeafDiagnostic::error()
            .with_code(LIBRARY_VALIDATION_ERROR)
            .with_message("Cannot validate library:")
            .with_labels(vec![LeafLabel::primary(
                ctx.get_current_file(),
                self.location.clone(),
            )
            .with_message(&self.message)])
    }

    fn should_print(&self, _: &DiagnosticsCtx) -> bool {
        true
    }
}
