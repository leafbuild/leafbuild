pub(crate) struct ExpectedTypeError {
    expected_type: String,
    loc_and_type: ExprLocAndType,
    docs_location: Option<String>,
}

impl ExpectedTypeError {
    pub(crate) fn new(expected_type: impl Into<String>, loc_and_type: ExprLocAndType) -> Self {
        Self {
            expected_type: expected_type.into(),
            loc_and_type,
            docs_location: None,
        }
    }

    pub(crate) fn with_docs_location(mut self, docs_location: impl Into<String>) -> Self {
        self.docs_location = Some(docs_location.into());
        self
    }

    pub(crate) fn with_docs_location_opt(mut self, docs: Option<String>) -> Self {
        self.docs_location = docs;
        self
    }
}

impl LeafDiagnosticTrait for ExpectedTypeError {
    fn get_diagnostic(self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        LeafDiagnostic::error()
            .with_message("Found expression with unexpected result")
            .with_code(EXPECTED_TYPE_ERROR)
            .with_labels(vec![LeafLabel::primary(
                ctx.get_current_file(),
                self.loc_and_type.loc,
            )
            .with_message(format!(
                "Expected {}, got {}",
                self.expected_type, self.loc_and_type.type_
            ))])
    }

    fn should_print(&self, _: &DiagnosticsCtx) -> bool {
        true
    }
}
