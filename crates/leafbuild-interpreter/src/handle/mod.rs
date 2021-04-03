/*
 *   Copyright (c) 2021 Dinu Blanovschi
 *   All rights reserved.
 *   Licensed under the terms of the BSD-3 Clause license, see LICENSE for more.
 */
//! A handle.
//! Explained in simple terms, this is a wrapper over all the structures the buildsystem uses internally.

// use crate::interpreter::{Env, EnvConfig};

use leafbuild_core::lf_buildsys::config::Config;
use leafbuild_core::lf_buildsys::{ConfigurationError, LfBuildsys, WriteResultsError};

/// The wrapper
#[derive(Debug)]
pub struct Handle<'a> {
    pub(crate) buildsys: LfBuildsys<'a>,

    validated: bool,
}

impl<'a> Handle<'a> {
    /// Constructor from the configuration
    #[must_use]
    pub fn new(cfg: Config) -> Self {
        Self {
            validated: false,
            buildsys: LfBuildsys::new(cfg),
        }
    }

    /// Validates the handle.
    /// # Errors
    /// See errors section of [`LfBuildsys::validate`]
    pub fn validate(&mut self) -> Result<&mut Self, ConfigurationError> {
        self.buildsys.validate()?;

        self.validated = true;

        Ok(self)
    }

    /// Writes the results stored in the environment
    ///
    /// **Important**: Should only be used after [`validation`][handle_validate].
    ///
    /// # Errors
    /// Any kind of error that can happen while writing, or if the buildsystem was not [`validate`][handle_validate]d yet.
    ///
    /// [handle_validate]: Handle::validate
    pub fn write_results(&self) -> Result<&Self, WriteResultsError> {
        self.buildsys.write_results()?;

        Ok(self)
    }
}
