//! Diagnostics in the build system
//! This is used almost everywhere.
use codespan_reporting::diagnostic::{Diagnostic, Label, LabelStyle, Severity};
use codespan_reporting::files;
use codespan_reporting::files::{Files, Location, SimpleFile};
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::ops::{Range, RangeInclusive};
use std::path::PathBuf;

/// The index of a file in the file database
#[derive(Debug, Copy, Clone)]
pub struct FileId {
    id: usize,
}

impl FileId {
    /// Creates a new [`FileId`] from the `id`
    #[must_use]
    pub const fn new(id: usize) -> Self {
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

/// A single file
pub type LeafbuildFile = SimpleFile<String, String>;

/// The file database
#[derive(Debug)]
pub struct LeafbuildFiles {
    files: Vec<LeafbuildFile>,
}

impl<'a> LeafbuildFiles {
    /// Adds a new file to the database and returns its [`FileId`]
    pub fn add(&'a mut self, name: String, source: String) -> FileId {
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

    fn name(&'a self, id: Self::FileId) -> Result<Self::Name, files::Error> {
        self.files
            .get(id.id)
            .map(LeafbuildFile::name)
            .ok_or(files::Error::FileMissing)
    }

    fn source(&'a self, id: Self::FileId) -> Result<Self::Source, files::Error> {
        self.files
            .get(id.id)
            .map(LeafbuildFile::source)
            .ok_or(files::Error::FileMissing)
    }

    fn line_index(&self, file_id: Self::FileId, byte_index: usize) -> Result<usize, files::Error> {
        self.files
            .get(file_id.id)
            .ok_or(files::Error::FileMissing)
            .and_then(|f| f.line_index((), byte_index))
    }

    fn line_range(
        &self,
        file_id: Self::FileId,
        line_index: usize,
    ) -> Result<Range<usize>, files::Error> {
        self.files
            .get(file_id.id)
            .ok_or(files::Error::FileMissing)
            .and_then(|f| f.line_range((), line_index))
    }
}

#[derive(Debug)]
struct LeafBuildTempFileContainer<'file> {
    file: SimpleFile<&'file str, &'file str>,
}

impl<'file> LeafBuildTempFileContainer<'file> {
    pub fn new(name: &'file str, source: &'file str) -> Self {
        Self {
            file: SimpleFile::new(name, source),
        }
    }
}

impl<'file> Files<'file> for LeafBuildTempFileContainer<'file> {
    type FileId = FileId;
    type Name = &'file str;
    type Source = &'file str;

    fn name(&'file self, _id: Self::FileId) -> Result<Self::Name, files::Error> {
        Ok(*self.file.name())
    }

    fn source(&'file self, _id: Self::FileId) -> Result<Self::Source, files::Error> {
        Ok(self.file.source())
    }

    fn line_index(
        &'file self,
        _id: Self::FileId,
        byte_index: usize,
    ) -> Result<usize, files::Error> {
        self.file.line_index((), byte_index)
    }

    fn line_number(
        &'file self,
        _id: Self::FileId,
        line_index: usize,
    ) -> Result<usize, files::Error> {
        self.file.line_number((), line_index)
    }

    fn column_number(
        &'file self,
        _id: Self::FileId,
        line_index: usize,
        byte_index: usize,
    ) -> Result<usize, files::Error> {
        self.file.column_number((), line_index, byte_index)
    }

    fn location(
        &'file self,
        _id: Self::FileId,
        byte_index: usize,
    ) -> Result<Location, files::Error> {
        self.file.location((), byte_index)
    }

    fn line_range(
        &'file self,
        _id: Self::FileId,
        line_index: usize,
    ) -> Result<Range<usize>, files::Error> {
        self.file.line_range((), line_index)
    }
}

/// the diagnostic type
#[derive(Debug)]
pub struct LeafDiagnostic {
    message: String,
    diagnostic_type: LeafDiagnosticType,
    diagnostic_code: usize,
    labels: Vec<LeafLabel>,
    notes: Vec<String>,
}

impl LeafDiagnostic {
    /// Creates a new diagnostic from the type
    #[must_use]
    pub fn new(diagnostic_type: LeafDiagnosticType) -> Self {
        Self {
            diagnostic_type,
            message: String::default(),
            diagnostic_code: usize::default(),
            labels: Vec::default(),
            notes: Vec::default(),
        }
    }

    /// Creates an error diagnostic
    #[must_use]
    pub fn error() -> Self {
        Self::new(LeafDiagnosticType::Error)
    }

    /// Creates a warning diagnostic
    #[must_use]
    pub fn warn() -> Self {
        Self::new(LeafDiagnosticType::Warn)
    }

    /// Sets the message
    #[must_use]
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Adds a [`LeafLabel`]
    #[must_use]
    pub fn with_label(mut self, label: impl Into<LeafLabel>) -> Self {
        self.labels.push(label.into());
        self
    }

    /// Sets the [`LeafLabel`]s
    #[must_use]
    pub fn with_labels(mut self, labels: Vec<LeafLabel>) -> Self {
        self.labels = labels;
        self
    }

    /// Pushes a single string as a note in this diagnostic
    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Sets the notes
    #[must_use]
    pub fn with_notes(mut self, notes: Vec<String>) -> Self {
        self.notes = notes;
        self
    }

    /// Sets the error/warning code
    #[must_use]
    pub const fn with_code(mut self, code: usize) -> Self {
        self.diagnostic_code = code;
        self
    }
}

impl From<LeafDiagnostic> for Diagnostic<FileId> {
    fn from(diagnostic: LeafDiagnostic) -> Self {
        Self::new(match diagnostic.diagnostic_type {
            LeafDiagnosticType::Error => Severity::Error,
            LeafDiagnosticType::Warn => Severity::Warning,
        })
        .with_message(diagnostic.message)
        .with_code(format!(
            "{}{}",
            match diagnostic.diagnostic_type {
                LeafDiagnosticType::Error => "E",
                LeafDiagnosticType::Warn => "W",
            },
            diagnostic.diagnostic_code
        ))
        .with_labels(
            diagnostic
                .labels
                .into_iter()
                .map(|label| label.into())
                .collect(),
        )
        .with_notes(diagnostic.notes)
    }
}

/// The diagnostic type
#[derive(Debug, Copy, Clone)]
pub enum LeafDiagnosticType {
    /// Is a warning
    Warn,
    /// Is an error
    Error,
}

/// The
pub trait LeafLabelLocation {
    /// Returns the start index of a [`LeafLabel`]
    fn get_start(&self) -> usize;
    /// Returns the end index of a [`LeafLabel`]
    fn get_end(&self) -> usize;
    /// Returns `get_start()..get_end()` as a [`Range`]
    fn get_range(&self) -> Range<usize> {
        self.get_start()..self.get_end()
    }
}

impl LeafLabelLocation for Range<usize> {
    fn get_start(&self) -> usize {
        self.start
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

impl LeafLabelLocation for RangeInclusive<usize> {
    fn get_start(&self) -> usize {
        *self.start()
    }

    fn get_end(&self) -> usize {
        *self.end() + 1
    }
}

/// The type of label
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LeafLabelType {
    /// A primary label.
    ///
    /// [`codespan_reporting`] uses `^` for primary labels
    Primary,
    /// A secondary label
    ///
    /// [`codespan_reporting`] uses `-` for secondary labels
    Secondary,
}

/// A label struct
#[derive(Debug)]
pub struct LeafLabel {
    file_id: FileId,
    label_type: LeafLabelType,
    location: Range<usize>,
    message: String,
}

impl LeafLabel {
    /// Create a primary label
    pub fn primary<T: LeafLabelLocation>(file_id: FileId, location: impl Borrow<T>) -> Self {
        Self {
            file_id,
            label_type: LeafLabelType::Primary,
            location: location.borrow().get_range(),
            message: String::default(),
        }
    }

    /// Create a secondary label
    pub fn secondary<T: LeafLabelLocation>(file_id: FileId, location: impl Borrow<T>) -> Self {
        Self {
            file_id,
            label_type: LeafLabelType::Secondary,
            location: location.borrow().get_range(),
            message: String::default(),
        }
    }

    /// Set the message
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
}

impl From<LeafLabel> for Label<FileId> {
    fn from(label: LeafLabel) -> Self {
        Self::new(
            match label.label_type {
                LeafLabelType::Primary => LabelStyle::Primary,
                LeafLabelType::Secondary => LabelStyle::Secondary,
            },
            label.file_id,
            label.location,
        )
        .with_message(label.message)
    }
}

/// The diagnostics configuration
#[derive(Debug, Clone)]
pub struct DiagConfig {
    error_eval_cascade: bool,
    log_location: PathBuf,
}

impl Default for DiagConfig {
    fn default() -> Self {
        Self {
            error_eval_cascade: false,
            log_location: PathBuf::from("leafbuild.log"),
        }
    }
}

/// The diagnostics context
#[derive(Default, Debug)]
pub struct DiagCtx {
    global_diagnostics_config: DiagConfig,
    files: LeafbuildFiles,
}

impl DiagCtx {
    /// Creates a new diagnostics context
    #[must_use]
    pub fn new(global_diagnostics_config: DiagConfig) -> Self {
        Self {
            global_diagnostics_config,
            files: LeafbuildFiles::default(),
        }
    }

    /// Reports the diagnostic
    pub fn report_diagnostic(&self, diagnostic: impl LeafDiagnosticTrait) {
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

    /// Add a new file to the file database
    pub fn add_file(&mut self, name: String, source: String) -> FileId {
        self.files.add(name, source)
    }

    /// Add a temporary file and call `f` with its id and the temporary diagnostics context
    pub fn with_temp_file<F>(&mut self, name: &str, source: &str, f: F)
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

/// A temporary diagnostics context.
///
/// Behaves and exposes the same api(see [`TempDiagnosticsContext::report_diagnostic`])
/// as a normal diagnostics context would
#[derive(Debug)]
pub struct TempDiagnosticsCtx<'a> {
    config: &'a DiagConfig,
    temp_file: LeafBuildTempFileContainer<'a>,
}

impl<'a> TempDiagnosticsCtx<'a> {
    /// See [`DiagCtx::report_diagnostic`]
    pub fn report_diagnostic(&self, diagnostic: impl LeafDiagnosticTrait) {
        if !diagnostic.should_report(self.config) {
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
pub trait LeafDiagnosticTrait {
    /// Converts `self` to `LeafDiagnostic`
    fn get_diagnostic(self) -> LeafDiagnostic;

    /// Specifies whether this diagnostic should be printed, given a diagnostics context `ctx`
    fn should_report(&self, ctx: &DiagConfig) -> bool;
}
