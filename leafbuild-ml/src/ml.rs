use leafbuild_core::lf_buildsys::LfBuildsys;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A simple result type with all the possible errors a middle layer might throw
pub type Result<T> = std::result::Result<T, MiddleLayerError>;

/// A middle layer trait
pub trait MiddleLayer {
    /// Try to recognize a given `add_subdir()`-ed directory.
    /// This usually should just check that `path` contains a
    /// given file meaningful to the build system of this middle
    /// layer.
    ///
    /// For example, for cmake: check that `path/CMakeLists.txt` exists.
    /// For meson, check that `path/meson.build` exists, etc.
    fn recognize(&self, path: &Path) -> RecognizeResult;

    /// Invokes the inner build system and returns the changes that should be made
    /// to the [`LfBuildsys`]
    /// # Errors
    /// Anything that can go wrong. Though the build will fail if this returns `Err`.
    fn handle<'buildsys>(
        &self,
        buildsys: &'buildsys LfBuildsys<'buildsys>,
        boundary_details: BuildsysBoundaryDetails,
    ) -> Result<BuildsysChanges>;
}

/// Whether the middle layer recognizes the directory or not
#[derive(Debug, Copy, Clone)]
pub enum RecognizeResult {
    /// When it was recognized
    Recognized,
    /// When it was not recognized
    NotRecognized,
}

/// The data passed across a
/// [build system boundary](https://leafbuild.github.com/dev/terminology.html#build-system-boundary)
#[derive(Debug)]
pub struct BuildsysBoundaryDetails<'boundary> {
    /// The source root
    pub source_root: &'boundary Path,
    /// The output root
    pub output_root: &'boundary Path,

    /// The source folder that was `add_subdir()`-ed, and checked
    /// previously with [`MiddleLayer::recognize`]
    pub source_folder: PathBuf,
    /// The output folder assigned to this subdirectory for the
    /// inner build system to write files to.
    pub output_folder: PathBuf,

    /// The "arguments" passed to the inner build system.
    /// In cmake they are variables that should
    /// be `set()` before invoking the `CMakeLists.txt`,
    /// while in meson they are global variables.
    ///
    /// Since meson takes types *somewhat* seriously,
    /// they should be converted.
    pub arguments: HashMap<String, String>,
    // will maybe add more
}

/// The build system changes that are to be applied to
/// a [`LfBuildsys`] after [`MiddleLayer::handle`]
/// executed.
#[derive(Copy, Clone, Debug, Default)]
pub struct BuildsysChanges {}

/// Errors that can occur during a middle layer's execution.
#[derive(Error, Debug)]
pub enum MiddleLayerError {
    /// Any other error
    #[error("Other error: {0}")]
    Other(#[from] Box<dyn std::error::Error>),
}
