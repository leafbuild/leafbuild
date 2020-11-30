pub enum LeafParseError<'error> {
    InvalidToken {
        location: usize,
        file_id: FileId,
    },
    UnrecognizedEOF {
        location: usize,
        expected: Vec<String>,
        file_id: FileId,
    },
    UnrecognizedToken {
        token: (usize, Token<'error>, usize),
        expected: Vec<String>,
        file_id: FileId,
    },
    ExtraToken {
        token: (usize, Token<'error>, usize),
        file_id: FileId,
    },
    Error {
        error: GrmError,
        file_id: FileId,
    },
}

impl<'error> LeafDiagnosticTrait for LeafParseError<'error> {
    fn get_diagnostic(self) -> LeafDiagnostic {
        match self {
            LeafParseError::InvalidToken { location, file_id } => LeafDiagnostic::error()
                .with_code(PARSE_ERROR)
                .with_message(format!("Invalid token at location {}", location))
                .with_label(LeafLabel::primary(file_id, location..=location).with_message("here")),
            LeafParseError::UnrecognizedEOF {
                location,
                expected,
                file_id,
            } => LeafDiagnostic::error()
                .with_code(PARSE_ERROR)
                .with_message("Unrecognized EOF")
                .with_label(LeafLabel::primary(file_id, location..=location).with_message("here"))
                .with_note(format!("Expected one of {:?}", expected)),
            LeafParseError::UnrecognizedToken {
                token,
                expected,
                file_id,
            } => LeafDiagnostic::error()
                .with_code(PARSE_ERROR)
                .with_message(format!("Unrecognized token {:?}", token.1))
                .with_label(LeafLabel::primary(file_id, token.0..token.2).with_message("here"))
                .with_note(format!("Expected one of {:?}", expected)),
            LeafParseError::ExtraToken { file_id, token } => LeafDiagnostic::error()
                .with_code(PARSE_ERROR)
                .with_message(format!("Extra token {:?}", token.1))
                .with_label(LeafLabel::primary(file_id, token.0..token.2).with_message("here")),
            LeafParseError::Error { file_id, error } => LeafDiagnostic::error()
                .with_code(PARSE_ERROR)
                .with_message(match error {
                    GrmError::LexError(ref err) => "Lexer error".into(),
                    GrmError::ParseIntError(ref err, _) => format!("Parsing number error: {}", err),
                })
                .with_label(
                    LeafLabel::primary(file_id, error.get_span().get_rng()).with_message("here"),
                ),
        }
    }

    fn should_report(&self, _config: &DiagConfig) -> bool {
        true
    }
}

impl<'error> From<(FileId, ParseError<usize, Token<'error>, GrmError>)> for LeafParseError<'error> {
    fn from((file_id, it): (FileId, ParseError<usize, Token<'error>, GrmError>)) -> Self {
        match it {
            ParseError::InvalidToken { location } => Self::InvalidToken { location, file_id },
            ParseError::UnrecognizedEOF { location, expected } => Self::UnrecognizedEOF {
                location,
                expected,
                file_id,
            },
            ParseError::UnrecognizedToken { token, expected } => Self::UnrecognizedToken {
                token,
                expected,
                file_id,
            },
            ParseError::ExtraToken { token } => Self::ExtraToken { token, file_id },
            ParseError::User { error } => Self::Error { error, file_id },
        }
    }
}
