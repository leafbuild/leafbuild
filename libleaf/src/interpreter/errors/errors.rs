use crate::interpreter::EnvFrame;
use codespan_reporting::{
    diagnostic::Diagnostic,
    files::SimpleFiles,
    term,
    term::termcolor::{ColorChoice, StandardStream},
};

/// the error context
pub(crate) struct ErrCtx {
    files: SimpleFiles<String, String>,
}

impl ErrCtx {
    pub(crate) fn new() -> Self {
        Self {
            files: SimpleFiles::new(),
        }
    }

    pub(crate) fn new_file(&mut self, name: String, src: String) -> usize {
        self.files.add(name, src)
    }
}

pub(crate) fn push_diagnostic(frame: &EnvFrame, diagnostic: Diagnostic<usize>) {
    let writer = StandardStream::stderr(ColorChoice::Auto);
    let config = codespan_reporting::term::Config::default();

    term::emit(
        &mut writer.lock(),
        &config,
        &frame.get_errctx().files,
        &diagnostic,
    )
    .unwrap();
}

pub(crate) fn error_message(message: &str, frame: &mut EnvFrame) -> String {
    #[cfg(feature = "angry-errors")]
    if frame.env_ref.angry_errors {
        return message.to_uppercase();
    }

    message.to_string()
}
