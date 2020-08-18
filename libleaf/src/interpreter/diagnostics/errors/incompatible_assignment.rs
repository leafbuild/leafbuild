pub(crate) struct IncompatibleAssignmentError {
    ref_location: Location,
    new_value_location: Location,
    old_value: String,
    old_value_type: String,
    new_value_type: String,
}

impl IncompatibleAssignmentError {
    pub(crate) fn new(
        ref_location: Location,
        new_value_location: Location,
        old_value: String,
        old_value_type: impl Into<String>,
        new_value_type: impl Into<String>,
    ) -> Self {
        Self {
            ref_location,
            new_value_location,
            old_value,
            old_value_type: old_value_type.into(),
            new_value_type: new_value_type.into(),
        }
    }
}

impl LeafDiagnosticTrait for IncompatibleAssignmentError {
    fn get_diagnostic(&self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        LeafDiagnostic::error()
            .with_message("Incompatible assignment")
            .with_code(INCOMPATIBLE_ASSIGNMENT_ERROR)
            .with_labels(vec![
                LeafLabel::primary(ctx.get_current_file(), self.ref_location.clone())
                    .with_message(self.old_value_type.clone()),
                LeafLabel::secondary(ctx.get_current_file(), self.new_value_location.clone())
                    .with_message(self.new_value_type.clone()),
            ])
            .with_notes(vec![format!(
                "it will keep it's previous value of `{}`",
                self.old_value
            )])
    }

    fn should_print(&self, _: &DiagnosticsCtx) -> bool {
        true
    }
}
