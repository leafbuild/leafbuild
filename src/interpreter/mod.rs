//! The interpreter module
//! Handles everything related to interpreting the source AST.
pub mod env;
mod internal;

use crate::interpreter::env::{ConfigurationError, WriteResultsError};
use crate::{diagnostics::errors::LeafParseError, grammar, handle::Handle};
use std::io;
use std::path::PathBuf;
use thiserror::Error;
use tracing::{span, Level};

include!("mod_name.rs");

/// Couldn't interpret something or validate something
#[derive(Error, Debug)]
pub enum InterpretFailure {
    /// Cannot read the source file
    #[error("cannot read file {0:?}: {1}")]
    CannotReadFile(PathBuf, #[source] io::Error),

    /// Cannot validate the configuration at the end.
    #[error(transparent)]
    Validate(#[from] ConfigurationError),

    /// Cannot write the results
    #[error(transparent)]
    CannotWriteResults(#[from] WriteResultsError),
}
/// Starts the interpreter on the given path, with the given handle and modifies the handle at the end.
/// The caller is responsible for validating and writing the results, by calling [`Handle::validate`]
/// and [`Handle::write_results`] after calling this.
/// # Errors
/// See [`InterpretFailure`]
pub fn execute_on<'a>(
    handle: &'a mut Handle<'a>,
    root_path: &PathBuf,
    mod_path: LfModName,
) -> Result<&'a mut Handle<'a>, InterpretFailure> {
    let span = span!(Level::TRACE, "execute_on", path = %mod_path.0.as_str());
    let _span_guard = span.enter();
    info!("Entered {}", mod_path.0.as_str());

    let build_decl_file = root_path.join("build.leaf");
    let content = std::fs::read_to_string(build_decl_file)
        .map_err(|err| InterpretFailure::CannotReadFile(root_path.join("build.leaf"), err))?;
    let mut errors = vec![];
    let result = grammar::parse(&content, &mut errors);

    handle.buildsys.register_file_and_report_chain(
        &root_path.to_string_lossy().to_string(),
        &content,
        |fid| {
            errors
                .into_iter()
                .map(move |err| LeafParseError::from((fid, err.error)))
        },
    );

    match result {
        Ok(build_definition) => {
            let fid = handle
                .buildsys
                .register_new_file(root_path.to_string_lossy().to_string(), content);
            let mut frame = env::FileFrame::new(fid, mod_path);
            build_definition
                .get_statements()
                .iter()
                .for_each(|statement| {
                    internal::run_statement(&mut frame, statement);
                });
        }
        Err(error) => {
            handle.buildsys.register_file_and_report(
                &root_path.to_string_lossy().to_string(),
                &content,
                |fid| LeafParseError::from((fid, error)),
            );
        }
    }

    info!("Leaving folder {:?}", root_path);

    Ok(&mut *handle)
}
