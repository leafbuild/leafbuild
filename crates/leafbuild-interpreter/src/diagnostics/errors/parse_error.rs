use leafbuild_syntax::parser::error::ParseError;
use leafbuild_syntax::TextSize;

pub struct LeafParseError {
    error: ParseError,
    position: TextSize,
    file_id: FileId,
}

impl LeafDiagnosticTrait for LeafParseError {
    fn get_diagnostic(self) -> LeafDiagnostic {
        LeafDiagnostic::error()
            .with_code(PARSE_ERROR)
            .with_message("Could not parse")
            .with_label(
                LeafLabel::primary(
                    self.file_id,
                    usize::from(self.position)..usize::from(self.position) + 1,
                )
                .with_message("here"),
            )
            .with_note(*self.error.0)
    }

    fn should_report(&self, _config: &DiagConfig) -> bool {
        true
    }
}

impl From<(FileId, (ParseError, TextSize))> for LeafParseError {
    fn from((file_id, (error, position)): (FileId, (ParseError, TextSize))) -> Self {
        Self {
            error,
            position,
            file_id,
        }
    }
}
