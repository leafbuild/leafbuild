//! The environment of the interpreter.
use crate::diagnostics::{DiagCtx, FileId, LeafDiagnosticTrait};
use crate::handle::config::Config;
use crate::interpreter::internal::fun;
use crate::interpreter::internal::values::Value;
use crate::interpreter::LfModName;
use derivative::Derivative;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::path::PathBuf;
use thiserror::Error;

/// The state of the buildsystem.
#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct LfBuildsys<'buildsys> {
    diagnostics_context: DiagCtx,
    output_directory: PathBuf,
    #[derivative(Debug = "ignore")]
    __phantom: ::std::marker::PhantomData<&'buildsys ()>,
}

/// Error while writing results (makefiles/`build.ninja` files ...).
#[derive(Debug, Error)]
pub enum WriteResultsError {
    /// An IO error occurred.
    #[error("IO: {0}")]
    IoError(#[from] std::io::Error),
}

/// Error in build system configuration.
#[derive(Debug, Error)]
pub enum ConfigurationError {
    /// Referenced a file that doesn't exist (maybe was deleted?)
    #[error("referenced file doesn't exit: {file}")]
    ReferencedFileDoesntExist {
        /// The file that was referenced.
        file: String,
    },
}

impl<'buildsys> LfBuildsys<'buildsys> {
    pub(crate) fn new(config: Config) -> Self {
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

    pub(crate) fn register_new_file(&mut self, name: String, source: String) -> FileId {
        self.diagnostics_context.add_file(name, source)
    }

    pub(crate) fn report_diagnostic(&self, diagnostic: impl LeafDiagnosticTrait) {
        self.diagnostics_context.report_diagnostic(diagnostic);
    }

    pub(crate) fn register_file_and_report<F, T>(&mut self, name: &str, source: &str, f: F)
    where
        F: FnOnce(FileId) -> T,
        T: LeafDiagnosticTrait,
    {
        self.diagnostics_context
            .with_temp_file(name, source, |ctx, file_id| {
                ctx.report_diagnostic(f(file_id))
            });
    }

    pub(crate) fn register_file_and_report_chain<F, It, T>(
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

    pub(crate) fn write_results(&self) -> Result<(), WriteResultsError> {
        std::fs::write(self.output_directory.join("build.ninja"), "")?;

        Ok(())
    }
}

/// A file frame, used to hold all the context information of a single file during execution,
/// For example names and values of variables and constants, declared types, functions, ....
#[derive(Debug)]
pub struct FileFrame<'frame> {
    file_id: FileId,
    mod_name: LfModName,
    __phantom: ::std::marker::PhantomData<&'frame ()>,
}

impl<'frame> FileFrame<'frame> {
    pub(crate) const fn new(file_id: FileId, mod_name: LfModName) -> Self {
        Self {
            file_id,
            mod_name,
            __phantom: PhantomData,
        }
    }
}

/// Name lookup data. A stack of those make up a file frame
#[derive(Debug)]
pub struct SemiFrame<'frame> {
    name_lookup: NameLookup<'frame>,

    parent_frame: Option<&'frame SemiFrame<'frame>>,
}

/// A name lookup table
#[derive(Debug)]
pub struct NameLookup<'frame> {
    variables: HashMap<String, Box<dyn Value<'frame>>>,
    functions: HashMap<String, fun::Fun>,
}
