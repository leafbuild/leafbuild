pub(crate) struct UnexpectedTypeInArray {
    vec_location: Location,
    found_type: String,
    expected_type: String,
    docs_location: Option<String>,
    index: usize,
}

impl UnexpectedTypeInArray {
    pub(crate) fn new(
        vec_location: Location,
        found_type: impl Into<String>,
        expected_type: impl Into<String>,
        index: usize,
    ) -> Self {
        Self {
            vec_location,
            found_type: found_type.into(),
            expected_type: expected_type.into(),
            docs_location: None,
            index,
        }
    }

    pub(crate) fn with_docs_location(mut self, docs: impl Into<String>) -> Self {
        self.docs_location = Some(docs.into());
        self
    }

    pub(crate) fn with_docs_location_opt(mut self, docs: Option<String>) -> Self {
        self.docs_location = docs;
        self
    }
}

impl LeafDiagnosticTrait for UnexpectedTypeInArray {
    fn get_diagnostic(&self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        let diagnostic = LeafDiagnostic::error()
            .with_code(UNEXPECTED_TYPE_IN_ARRAY_ERROR)
            .with_message(format!(
                "Found a{} {} in an array that was supposed to only contain {}s",
                (match self.found_type.chars().next().unwrap() {
                    'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => "n",
                    _ => "",
                }),
                self.found_type,
                self.expected_type
            ))
            .with_labels(vec![LeafLabel::primary(
                ctx.current_file,
                self.vec_location.clone(),
            )
            .with_message(format!("here, at index {}", self.index))]);
        match &self.docs_location {
            Some(loc) => {
                diagnostic.with_notes(vec![format!("Documentation at {}{}", DOCS_ROOT, loc)])
            }
            None => diagnostic,
        }
    }

    fn should_print(&self, _: &DiagnosticsCtx) -> bool {
        true
    }
}
