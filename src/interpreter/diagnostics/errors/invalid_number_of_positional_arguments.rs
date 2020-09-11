pub(crate) struct InvalidNumberOfPositionalArguments {
    name_token_location: Location,
    positional_args_loc: Location,
    required: usize,
    found: usize,
    docs_location: Option<String>,
}

impl InvalidNumberOfPositionalArguments {
    pub(crate) fn new(
        name_token_location: Location,
        positional_args_loc: Location,
        required: usize,
        found: usize,
    ) -> Self {
        Self {
            name_token_location,
            positional_args_loc,
            required,
            found,
            docs_location: None,
        }
    }

    pub(crate) fn with_docs_location(mut self, location: impl Into<String>) -> Self {
        self.docs_location = Some(location.into());
        self
    }
}

impl LeafDiagnosticTrait for InvalidNumberOfPositionalArguments {
    fn get_diagnostic(&self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        match (
            self.docs_location.clone(),
            LeafDiagnostic::error()
                .with_message("Invalid number of positional arguments passed to function/method")
                .with_code(INVALID_NUMBER_OF_POSITIONAL_ARGUMENTS)
                .with_labels(vec![
                    LeafLabel::primary(ctx.get_current_file(), self.positional_args_loc.clone())
                        .with_message(format!("Got {} positional arguments", self.found)),
                    LeafLabel::secondary(ctx.get_current_file(), self.name_token_location.clone())
                        .with_message(format!("Needed {}", self.required)),
                ]),
        ) {
            (Some(loc), diagnostic) => diagnostic.with_notes(vec![format!(
                "Documentation of function/method at {}{}",
                DOCS_ROOT, loc
            )]),
            (None, diagnostic) => diagnostic,
        }
    }

    fn should_print(&self, _: &DiagnosticsCtx) -> bool {
        true
    }
}
