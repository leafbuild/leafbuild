pub(crate) struct VarNameInPrelude {
    location: Location,
}

impl VarNameInPrelude {
    pub(crate) fn new(location: Location) -> Self {
        Self { location }
    }
}

impl LeafDiagnosticTrait for VarNameInPrelude {
    fn get_diagnostic(&self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        LeafDiagnostic::warn()
            .with_code(VAR_NAME_IN_PRELUDE)
            .with_message(
                "Won't create this variable because the name is already used in the prelude",
            )
            .with_labels(vec![LeafLabel::primary(
                ctx.current_file,
                self.location.clone(),
            )
            .with_message("name here")])
            .with_notes(vec!["Consider changing the name".to_string()])
    }

    fn should_print(&self, _: &DiagnosticsCtx) -> bool {
        true
    }
}
