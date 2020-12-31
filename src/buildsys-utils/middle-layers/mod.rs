//! The middle-layers
//! Used to generate cmake / meson and other build systems from the original leafbuild system.
//! An "IR" between the target `make` / `ninja` / ... and the `leafbuild` layer.
//!
//! Most of them will be optional.

use leafbuild_interpreter::env::LfBuildsys;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[cfg(feature = "cmake-ml")]
pub mod cmake_ml;

#[cfg(feature = "meson-ml")]
pub mod meson_ml;

///
#[derive(Error, Debug)]
pub enum MiddleLayerOutputError {
    /// An io error
    #[error("An io error occurred: {0}")]
    IoError(#[from] io::Error),
}

///
pub trait MiddleLayer {
    /// The error output of the validation
    type ValidateError: Error;

    /// The validation action
    /// Note that this function is called after the buildsystem was initially validated
    /// The checks leafbuild performs on the buildsystem to validate it are described [here](https://leafbuild.github.io/validation_checks.html)
    ///
    /// # Errors
    /// Returns any errors that may prevent the `buildsys` from being processed properly by the middle one.
    fn validate(buildsys: &LfBuildsys) -> Result<(), Self::ValidateError>;

    /// Outputs the previously-validated by [`MiddleLayer::validate`] `buildsys` to the given `path`, returning Ok(())
    /// # Errors
    ///
    /// IO errors,
    fn output(buildsys: LfBuildsys, path: PathBuf) -> Result<(), MiddleLayerOutputError>;
}
