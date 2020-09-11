/// An exception shown when evaluating a map, in case we met a key that was already in the map
/// For example:
/// ```leafbuild
/// {a: 1, a: 2}
/// ```

pub(crate) struct KeyAlreadyInMap<'a> {
    /// the name of the duplicated key
    key_name: &'a str,
    /// The location of the map
    location: Location,
    /// The location of the pair of the duplicated key
    key_value_pair_location: Location,
}

impl<'a> KeyAlreadyInMap<'a> {
    pub(crate) fn new(
        key_name: &'a str,
        location: Location,
        key_value_pair_location: Location,
    ) -> Self {
        Self {
            key_name,
            location,
            key_value_pair_location,
        }
    }
}

impl<'a> LeafDiagnosticTrait for KeyAlreadyInMap<'a> {
    fn get_diagnostic(&self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        LeafDiagnostic::warn()
            .with_code(KEY_ALREADY_IN_MAP)
            .with_message(format!(
                "Key '{}' was already added to the map",
                self.key_name
            ))
            .with_labels(vec![
                LeafLabel::primary(ctx.get_current_file(), self.location.clone())
                    .with_message("Map here"),
                LeafLabel::secondary(ctx.get_current_file(), self.key_value_pair_location.clone())
                    .with_message("Duplicated key here"),
            ])
    }

    fn should_print(&self, _: &DiagnosticsCtx) -> bool {
        true
    }
}
