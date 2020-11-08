use crate::diagnostics::{DiagCtx, FileId, LeafDiagnosticTrait};
use crate::handle::config::Config;
use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Default)]
pub struct Env<'env> {
    diagnostics_context: DiagCtx,
    output_directory: PathBuf,
    __phantom: ::std::marker::PhantomData<&'env ()>,
}

impl<'env> Env<'env> {
    pub(crate) fn new(config: Config) -> Self {
        Self {
            diagnostics_context: DiagCtx::new(config.diagnostics_config),
            output_directory: config.output_directory,
            __phantom: PhantomData,
        }
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

    pub(crate) fn write_results(&self) -> std::io::Result<()> {
        std::fs::write(self.output_directory.join("build.ninja"), "")
    }
}

pub struct FileFrame<'frame> {
    file_id: FileId,
    __phantom: ::std::marker::PhantomData<&'frame ()>,
}

impl<'frame> FileFrame<'frame> {
    pub(crate) const fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            __phantom: PhantomData,
        }
    }
}
