use crate::interpreter::EnvFrame;
use codespan_reporting::{
    diagnostic::Diagnostic,
    files::SimpleFiles,
    term,
    term::termcolor::{ColorChoice, StandardStream},
};
use std::ops::Range;

/// the error context
pub(crate) struct ErrCtx {
    files: SimpleFiles<String, String>,
    angry_errors: bool,
    error_cascade: bool,
}

pub(crate) type Location = Range<usize>;

impl ErrCtx {
    pub(crate) fn new(angry_errors: bool, error_cascade: bool) -> Self {
        Self {
            files: SimpleFiles::new(),
            angry_errors,
            error_cascade,
        }
    }

    pub(crate) fn new_file(&mut self, name: String, src: String) -> usize {
        self.files.add(name, src)
    }

    pub(crate) fn get_error_cascade(&self) -> bool {
        self.error_cascade
    }
}

#[inline]
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

#[inline]
pub(crate) fn push_diagnostic_ctx(errctx: &ErrCtx, diagnostic: Diagnostic<usize>) {
    let writer = StandardStream::stderr(ColorChoice::Auto);
    let config = codespan_reporting::term::Config::default();

    term::emit(&mut writer.lock(), &config, &errctx.files, &diagnostic).unwrap();
}

#[inline]
pub(crate) fn get_error(txt: &str, frame: &EnvFrame) -> String {
    #[cfg(feature = "angry-errors")]
    if frame.env_ref.errctx.angry_errors {
        return txt.to_uppercase();
    }
    txt.to_string()
}

#[inline]
pub(crate) fn get_error_string(txt: String, frame: &EnvFrame) -> String {
    #[cfg(feature = "angry-errors")]
    if frame.env_ref.errctx.angry_errors {
        return txt.to_uppercase();
    }
    txt
}

#[inline]
pub(crate) fn get_error_ctx(txt: String, ctx: &ErrCtx) -> String {
    #[cfg(feature = "angry-errors")]
    if ctx.angry_errors {
        return txt.to_uppercase();
    }
    txt
}
