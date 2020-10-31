use crate::diagnostics::{DiagnosticsCtx, FileId, LeafDiagnosticTrait};
use crate::handle::config::EnvConfig;
use std::marker::PhantomData;

#[derive(Default)]
pub(crate) struct Env<'env> {
    diagnostics_context: DiagnosticsCtx,
    __phantom: ::std::marker::PhantomData<&'env ()>,
}

impl<'env> Env<'env> {
    pub(crate) fn new(config: EnvConfig) -> Self {
        Self {
            diagnostics_context: DiagnosticsCtx::new(config.diagnostics_config),
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
}

pub(crate) struct EnvFrame<'frame> {
    file_id: FileId,
    __phantom: ::std::marker::PhantomData<&'frame ()>,
}

impl<'frame> EnvFrame<'frame> {
    pub(crate) fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            __phantom: PhantomData,
        }
    }
}
