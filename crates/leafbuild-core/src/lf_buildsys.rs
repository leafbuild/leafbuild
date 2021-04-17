//! [`LfBuildsys`] and stuff related to it.
pub mod config;
use crate::diagnostics::{DiagCtx, FileId, LeafDiagnosticTrait};
use config::Config;
use std::marker::PhantomData;
use std::path::PathBuf;

/// The state of the buildsystem.
#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct LfBuildsys<'buildsys> {
    diagnostics_context: DiagCtx,
    output_directory: PathBuf,
    #[derivative(Debug = "ignore")]
    __phantom: PhantomData<&'buildsys ()>,
}

/// Error while writing results (makefiles/`build.ninja` files ...).
#[derive(Debug, Error)]
pub enum WriteResultsError {
    /// The build system was not validated.
    #[error("Not validated")]
    NotValidated,
    /// An IO error occurred.
    #[error("IO error")]
    IoError(#[from] std::io::Error),
}

/// Error in build system configuration.
#[derive(Debug, Error)]
pub enum ConfigurationError {
    /// Referenced a file that doesn't exist (maybe was deleted?)
    #[error("referenced file doesn't exit: {file}")]
    ReferencedFileDoesntExist {
        /// The file that was referenced.
        file: PathBuf,
    },
}

impl<'buildsys> LfBuildsys<'buildsys> {
    /// Create a new buildsystem from the given initial configuration
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self {
            diagnostics_context: DiagCtx::new(config.diagnostics_config),
            output_directory: config.output_directory,
            __phantom: PhantomData,
        }
    }

    /// Validates the build system. This is done at the end, and is a prerequisite
    /// to writing the results.
    ///
    /// # Errors
    ///
    /// Any configuration errors.
    #[allow(clippy::unused_self)] // TODO: work out validation
    pub const fn validate(&self) -> Result<(), ConfigurationError> {
        Ok(())
    }

    /// Registers a new file to the file database and returns its id
    pub fn register_new_file(&mut self, name: String, source: String) -> FileId {
        self.diagnostics_context.add_file(name, source)
    }

    /// Writes the results
    /// # Errors
    /// Any errors that can happen while writing a *valid* [`LfBuildsys`]\(validated by [`LfBuildsys::validate`])
    pub fn write_results(&self) -> Result<(), WriteResultsError> {
        std::fs::write(self.output_directory.join("build.ninja"), "")?;

        Ok(())
    }

    /// Reports the given diagnostic
    pub fn report_diagnostic(&self, diagnostic: impl LeafDiagnosticTrait) {
        self.diagnostics_context.report_diagnostic(diagnostic);
    }

    /// Registers a file and reports a diagnostic with it
    pub fn register_file_and_report<F, T>(&mut self, name: &str, source: &str, f: F)
    where
        F: FnOnce(FileId) -> T,
        T: LeafDiagnosticTrait,
    {
        self.diagnostics_context
            .with_temp_file(name, source, |ctx, file_id| {
                ctx.report_diagnostic(f(file_id))
            });
    }

    /// Registers a file and reports all the diagnostics in the iterator returned by `diagnostics`
    pub fn register_file_and_report_chain<F, It, T>(
        &mut self,
        name: &str,
        source: &str,
        diagnostics: F,
    ) where
        F: FnOnce(FileId) -> It,
        It: Iterator<Item = T>,
        T: LeafDiagnosticTrait,
    {
        self.diagnostics_context
            .with_temp_file(name, source, |ctx, file_id| {
                diagnostics(file_id).for_each(|diagnostic| ctx.report_diagnostic(diagnostic));
            });
    }
}
