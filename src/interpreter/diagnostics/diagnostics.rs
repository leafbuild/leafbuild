#[path = "errors/errors.rs"]
pub(crate) mod errors;
#[path = "warnings/warnings.rs"]
pub(crate) mod warnings;

use crate::interpreter::EnvFrame;
use codespan_reporting::diagnostic::{Label, LabelStyle, Severity};
use codespan_reporting::{
    diagnostic::Diagnostic,
    files::SimpleFiles,
    term,
    term::termcolor::{ColorChoice, StandardStream},
};
use std::ops::{Deref, Range};

/// the diagnostic type
pub(crate) struct LeafDiagnostic {
    message: String,
    diagnostic_type: LeafDiagnosticType,
    diagnostic_code: usize,
    labels: Vec<LeafLabel>,
    notes: Vec<String>,
}

/// A diagnostic - basically a thin wrapper over the `codespan_reporting` `Diagnostic<usize>`
impl LeafDiagnostic {
    pub(crate) fn new(diagnostic_type: LeafDiagnosticType) -> LeafDiagnostic {
        LeafDiagnostic {
            diagnostic_type,
            message: String::default(),
            diagnostic_code: usize::default(),
            labels: Vec::default(),
            notes: Vec::default(),
        }
    }

    pub(crate) fn error() -> LeafDiagnostic {
        Self::new(LeafDiagnosticType::Error)
    }

    pub(crate) fn warn() -> LeafDiagnostic {
        Self::new(LeafDiagnosticType::Warn)
    }

    pub(crate) fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub(crate) fn with_labels(mut self, labels: Vec<LeafLabel>) -> Self {
        self.labels = labels;
        self
    }

    pub(crate) fn with_notes(mut self, notes: Vec<String>) -> Self {
        self.notes = notes;
        self
    }

    pub(crate) fn with_code(mut self, code: usize) -> Self {
        self.diagnostic_code = code;
        self
    }
}

impl Into<Diagnostic<usize>> for LeafDiagnostic {
    fn into(self) -> Diagnostic<usize> {
        Diagnostic::new(match self.diagnostic_type {
            LeafDiagnosticType::Error => Severity::Error,
            LeafDiagnosticType::Warn => Severity::Warning,
        })
        .with_message(self.message)
        .with_code(format!(
            "{}{}",
            match self.diagnostic_type {
                LeafDiagnosticType::Error => "E",
                LeafDiagnosticType::Warn => "W",
            },
            self.diagnostic_code
        ))
        .with_labels(self.labels.into_iter().map(|label| label.into()).collect())
        .with_notes(self.notes)
    }
}

pub(crate) enum LeafDiagnosticType {
    Warn,
    Error,
}

type LeafLabelFileId = usize;
type LeafLabelLocation = Range<usize>;

pub(crate) enum LeafLabelType {
    Primary,
    Secondary,
}

pub(crate) struct LeafLabel {
    file_id: LeafLabelFileId,
    label_type: LeafLabelType,
    location: LeafLabelLocation,
    message: String,
}

impl LeafLabel {
    pub(crate) fn primary(file_id: LeafLabelFileId, location: LeafLabelLocation) -> Self {
        Self {
            file_id,
            label_type: LeafLabelType::Primary,
            location,
            message: String::default(),
        }
    }

    pub(crate) fn secondary(file_id: LeafLabelFileId, location: LeafLabelLocation) -> Self {
        Self {
            file_id,
            label_type: LeafLabelType::Secondary,
            location,
            message: String::default(),
        }
    }

    pub(crate) fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
}

impl Into<Label<usize>> for LeafLabel {
    fn into(self) -> Label<usize> {
        Label::new(
            match self.label_type {
                LeafLabelType::Primary => LabelStyle::Primary,
                LeafLabelType::Secondary => LabelStyle::Secondary,
            },
            self.file_id,
            self.location,
        )
        .with_message(self.message)
    }
}

/// Basically a thing that can be converted into the `LeafDiagnostic` type above
pub(crate) trait LeafDiagnosticTrait {
    /// Converts `&self` to `LeafDiagnostic`
    fn get_diagnostic(&self, ctx: &DiagnosticsCtx) -> LeafDiagnostic;

    /// Specifies whether this diagnostic should be printed, given a diagnostics context `ctx`
    fn should_print(&self, ctx: &DiagnosticsCtx) -> bool;
}

impl<T> LeafDiagnosticTrait for Box<T>
where
    T: LeafDiagnosticTrait + ?Sized,
{
    fn get_diagnostic(&self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        T::get_diagnostic(self.deref(), ctx)
    }

    fn should_print(&self, ctx: &DiagnosticsCtx) -> bool {
        T::should_print(self.deref(), ctx)
    }
}

/// The diagnostics context
/// Basically has some data about the diagnostics-related commandline options and index of the
/// current file we are in, as well as owns all the `codespan_reporting` files.
pub(crate) struct DiagnosticsCtx {
    files: SimpleFiles<String, String>,
    angry_errors: bool,
    error_cascade: bool,
    signal_build_failure: bool,

    current_file: usize,
}

pub(crate) type Location = LeafLabelLocation;

impl DiagnosticsCtx {
    pub(crate) fn new(angry_errors: bool, error_cascade: bool, signal_build_failure: bool) -> Self {
        Self {
            files: SimpleFiles::new(),
            angry_errors,
            error_cascade,
            signal_build_failure,
            current_file: 0,
        }
    }

    pub(crate) fn new_file(&mut self, name: String, src: String) -> usize {
        self.current_file = self.files.add(name, src);
        self.current_file
    }

    pub(crate) fn get_error_cascade(&self) -> bool {
        self.error_cascade
    }

    pub(crate) fn get_current_file(&self) -> usize {
        self.current_file
    }
    pub(crate) fn set_current_file(&mut self, current_file: usize) {
        self.current_file = current_file;
    }

    pub(crate) fn get_signal_build_failure(&self) -> bool {
        self.signal_build_failure
    }
}

pub(crate) fn push_diagnostic(diagnostic: impl LeafDiagnosticTrait, frame: &EnvFrame) {
    push_diagnostic_ctx(diagnostic, frame.get_diagnostics_ctx())
}

pub(crate) fn push_diagnostic_ctx(diagnostic: impl LeafDiagnosticTrait, ctx: &DiagnosticsCtx) {
    if !diagnostic.should_print(ctx) {
        return;
    }
    let writer = StandardStream::stderr(ColorChoice::Auto);
    let config = codespan_reporting::term::Config::default();

    term::emit(
        &mut writer.lock(),
        &config,
        &ctx.files,
        &diagnostic.get_diagnostic(ctx).into(),
    )
    .unwrap();
}
