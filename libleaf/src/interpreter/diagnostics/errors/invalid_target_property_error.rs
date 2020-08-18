pub(crate) struct InvalidTargetPropertyError<'a> {
    msg: InvalidTargetProperty,
    location: Location,
    key_name: &'a String,
    value: &'a Value<Box<dyn ValueTypeMarker>>,
}

impl<'a> InvalidTargetPropertyError<'a> {
    pub(crate) fn new(
        msg: InvalidTargetProperty,
        location: Location,
        key_name: &'a String,
        value: &'a Value<Box<dyn ValueTypeMarker>>,
    ) -> Self {
        Self {
            msg,
            location,
            key_name,
            value,
        }
    }
}

impl<'a> LeafDiagnosticTrait for InvalidTargetPropertyError<'a> {
    fn get_diagnostic(&self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        LeafDiagnostic::error()
            .with_message(self.msg.get_message().clone())
            .with_code(INVALID_TARGET_PROPERTY_ERROR)
            .with_labels(vec![LeafLabel::primary(
                ctx.get_current_file(),
                self.location.clone(),
            )
            .with_message(format!(
                "here(key: {}, value: {})",
                self.key_name,
                self.value.stringify()
            ))])
    }

    fn should_print(&self, _: &DiagnosticsCtx) -> bool {
        true
    }
}
