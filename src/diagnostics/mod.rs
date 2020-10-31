pub(crate) mod errors;
pub(crate) mod warnings;

use codespan_reporting::diagnostic::{Diagnostic, Label, LabelStyle, Severity};
use codespan_reporting::files::{Files, Location, SimpleFile};
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use std::cmp::Ordering;
use std::ops::Range;

#[derive(Copy, Clone)]
pub struct FileId {
    id: usize,
}

impl FileId {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

impl PartialEq for FileId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for FileId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }

    fn lt(&self, other: &Self) -> bool {
        self.id.lt(&other.id)
    }

    fn le(&self, other: &Self) -> bool {
        self.id.le(&other.id)
    }

    fn gt(&self, other: &Self) -> bool {
        self.id.gt(&other.id)
    }

    fn ge(&self, other: &Self) -> bool {
        self.id.ge(&other.id)
    }
}

pub type LeafbuildFile = SimpleFile<String, String>;

pub struct LeafbuildFiles {
    files: Vec<LeafbuildFile>,
}

impl<'a> LeafbuildFiles {
    pub(crate) fn add(&'a mut self, name: String, source: String) -> FileId {
        self.files.push(LeafbuildFile::new(name, source));
        FileId::new(self.files.len() - 1)
    }
}

impl Default for LeafbuildFiles {
    fn default() -> Self {
        Self { files: vec![] }
    }
}

impl<'a> Files<'a> for LeafbuildFiles {
    type FileId = FileId;
    type Name = &'a String;
    type Source = &'a String;

    fn name(&'a self, id: Self::FileId) -> Option<Self::Name> {
        self.files.get(id.id).map(LeafbuildFile::name)
    }

    fn source(&'a self, id: Self::FileId) -> Option<Self::Source> {
        self.files.get(id.id).map(LeafbuildFile::source)
    }

    fn line_index(&self, file_id: Self::FileId, byte_index: usize) -> Option<usize> {
        self.files.get(file_id.id)?.line_index((), byte_index)
    }

    fn line_range(&self, file_id: Self::FileId, line_index: usize) -> Option<Range<usize>> {
        self.files.get(file_id.id)?.line_range((), line_index)
    }
}

struct LeafBuildTempFileContainer<'file> {
    file: SimpleFile<&'file str, &'file str>,
}

impl<'file> LeafBuildTempFileContainer<'file> {
    pub(crate) fn new(name: &'file str, source: &'file str) -> Self {
        Self {
            file: SimpleFile::new(name, source),
        }
    }
}

impl<'file> Files<'file> for LeafBuildTempFileContainer<'file> {
    type FileId = FileId;
    type Name = &'file str;
    type Source = &'file str;

    fn name(&'file self, _id: Self::FileId) -> Option<Self::Name> {
        Some(self.file.name())
    }

    fn source(&'file self, _id: Self::FileId) -> Option<Self::Source> {
        Some(self.file.source())
    }

    fn line_index(&'file self, _id: Self::FileId, byte_index: usize) -> Option<usize> {
        self.file.line_index((), byte_index)
    }

    fn line_number(&'file self, _id: Self::FileId, line_index: usize) -> Option<usize> {
        self.file.line_number((), line_index)
    }

    fn column_number(
        &'file self,
        _id: Self::FileId,
        line_index: usize,
        byte_index: usize,
    ) -> Option<usize> {
        self.file.column_number((), line_index, byte_index)
    }

    fn location(&'file self, _id: Self::FileId, byte_index: usize) -> Option<Location> {
        self.file.location((), byte_index)
    }

    fn line_range(&'file self, _id: Self::FileId, line_index: usize) -> Option<Range<usize>> {
        self.file.line_range((), line_index)
    }
}

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

    pub(crate) fn with_label(mut self, label: impl Into<LeafLabel>) -> Self {
        self.labels.push(label.into());
        self
    }

    pub(crate) fn with_labels(mut self, labels: Vec<LeafLabel>) -> Self {
        self.labels = labels;
        self
    }

    pub(crate) fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
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

impl Into<Diagnostic<FileId>> for LeafDiagnostic {
    fn into(self) -> Diagnostic<FileId> {
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

type LeafLabelLocation = Range<usize>;

pub(crate) enum LeafLabelType {
    Primary,
    Secondary,
}

pub(crate) struct LeafLabel {
    file_id: FileId,
    label_type: LeafLabelType,
    location: LeafLabelLocation,
    message: String,
}

impl LeafLabel {
    pub(crate) fn primary(file_id: FileId, location: LeafLabelLocation) -> Self {
        Self {
            file_id,
            label_type: LeafLabelType::Primary,
            location,
            message: String::default(),
        }
    }

    pub(crate) fn secondary(file_id: FileId, location: LeafLabelLocation) -> Self {
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

impl Into<Label<FileId>> for LeafLabel {
    fn into(self) -> Label<FileId> {
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

#[derive(Default, Clone)]
pub(crate) struct DiagnosticsConfig {
    error_eval_cascade: bool,
}

#[derive(Default)]
pub(crate) struct DiagnosticsCtx {
    global_diagnostics_config: DiagnosticsConfig,
    files: LeafbuildFiles,
}

impl DiagnosticsCtx {
    pub(crate) fn new(global_diagnostics_config: DiagnosticsConfig) -> DiagnosticsCtx {
        Self {
            global_diagnostics_config,
            files: LeafbuildFiles::default(),
        }
    }
    pub(crate) fn report_diagnostic(&self, diagnostic: impl LeafDiagnosticTrait) {
        if !diagnostic.should_report(&self.global_diagnostics_config) {
            return;
        }
        let writer = StandardStream::stderr(ColorChoice::Auto);
        let config = codespan_reporting::term::Config::default();

        codespan_reporting::term::emit(
            &mut writer.lock(),
            &config,
            &self.files,
            &diagnostic.get_diagnostic().into(),
        )
        .unwrap();
    }
    pub(crate) fn add_file(&mut self, name: String, source: String) -> FileId {
        self.files.add(name, source)
    }
    pub(crate) fn with_temp_file<F>(&mut self, name: &str, source: &str, f: F)
    where
        F: FnOnce(TempDiagnosticsCtx, FileId),
    {
        let file = LeafBuildTempFileContainer::new(name, source);
        // file id doesn't matter since it's never used.
        f(self.temp_context(file), FileId::new(0))
    }
    fn temp_context<'a>(
        &'a mut self,
        temp_file: LeafBuildTempFileContainer<'a>,
    ) -> TempDiagnosticsCtx<'a> {
        TempDiagnosticsCtx {
            config: &self.global_diagnostics_config,
            temp_file,
        }
    }
}

pub(crate) struct TempDiagnosticsCtx<'a> {
    config: &'a DiagnosticsConfig,
    temp_file: LeafBuildTempFileContainer<'a>,
}

impl<'a> TempDiagnosticsCtx<'a> {
    pub(crate) fn report_diagnostic(&self, diagnostic: impl LeafDiagnosticTrait) {
        if !diagnostic.should_report(&self.config) {
            return;
        }
        let writer = StandardStream::stderr(ColorChoice::Auto);
        let config = codespan_reporting::term::Config::default();

        codespan_reporting::term::emit(
            &mut writer.lock(),
            &config,
            &self.temp_file,
            &diagnostic.get_diagnostic().into(),
        )
        .unwrap();
    }
}

/// Basically a thing that can be converted into the `LeafDiagnostic` type above
pub(crate) trait LeafDiagnosticTrait {
    /// Converts `self` to `LeafDiagnostic`
    fn get_diagnostic(self) -> LeafDiagnostic;

    /// Specifies whether this diagnostic should be printed, given a diagnostics context `ctx`
    fn should_report(&self, ctx: &DiagnosticsConfig) -> bool;
}
