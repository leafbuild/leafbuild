pub(crate) struct InvalidIndexBaseError {
    loc_and_type: ExprLocAndType,
}

impl InvalidIndexBaseError {
    pub(crate) fn new(loc_and_type: ExprLocAndType) -> Self {
        Self { loc_and_type }
    }
}

impl LeafDiagnosticTrait for InvalidIndexBaseError {
    fn get_diagnostic(self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        LeafDiagnostic::error()
            .with_message("Invalid base for indexed expression")
            .with_code(INVALID_INDEX_BASE_ERROR)
            .with_labels(vec![LeafLabel::primary(
                ctx.get_current_file(),
                self.loc_and_type.loc,
            )
            .with_message(self.loc_and_type.type_)])
    }

    fn should_print(&self, _: &DiagnosticsCtx) -> bool {
        true
    }
}
