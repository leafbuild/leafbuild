pub(crate) struct UnknownPropertyError {
    base: ExprLocAndType,
    property_name: String,
    property_location: Location,
}

impl UnknownPropertyError {
    pub(crate) fn new(
        base: ExprLocAndType,
        property_name: impl Into<String>,
        property_location: Location,
    ) -> UnknownPropertyError {
        Self {
            base,
            property_name: property_name.into(),
            property_location,
        }
    }
}

impl LeafDiagnosticTrait for UnknownPropertyError {
    fn get_diagnostic(&self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        LeafDiagnostic::error()
            .with_message(format!(
                "Trying to access unknown property `{}' on type `{}'",
                self.property_name, self.base.type_
            ))
            .with_code(UNKNOWN_PROPERTY_ERROR)
            .with_labels(vec![
                LeafLabel::primary(ctx.get_current_file(), self.property_location.clone())
                    .with_message("here"),
                LeafLabel::secondary(ctx.get_current_file(), self.base.loc.clone())
                    .with_message(format!("base here, of type `{}'", self.base.type_)),
            ])
    }

    fn should_print(&self, _: &DiagnosticsCtx) -> bool {
        true
    }
}
