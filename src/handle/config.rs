use crate::diagnostics::DiagnosticsConfig;
use std::path::PathBuf;

#[derive(Clone)]
pub struct EnvConfig {
    error_cascade: bool,
    output_directory: PathBuf,
    signal_build_failure: bool,
    pub(crate) diagnostics_config: DiagnosticsConfig,
}

impl EnvConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_error_cascade(mut self, error_cascade: bool) -> Self {
        self.error_cascade = error_cascade;
        self
    }

    pub fn with_output_directory(mut self, output_directory: PathBuf) -> Self {
        self.output_directory = output_directory;
        self
    }

    pub fn with_signal_build_failure(mut self, signal_build_failure: bool) -> Self {
        self.signal_build_failure = signal_build_failure;
        self
    }
}

impl Default for EnvConfig {
    fn default() -> Self {
        Self {
            error_cascade: true,
            output_directory: PathBuf::from("leafbuild-dir"),
            signal_build_failure: false,
            diagnostics_config: DiagnosticsConfig::default(),
        }
    }
}
