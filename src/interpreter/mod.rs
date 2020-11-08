pub(crate) mod env;
mod internal;

use crate::{diagnostics::errors::LeafParseError, grammar, handle::Handle};
use std::{error::Error, path::PathBuf};

/// Starts the interpreter on the given path, with the given handle and modifies the handle at the end.
/// # Errors
/// Anything
pub fn start_on<'a>(handle: &'a mut Handle<'a>, root_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    info!("Entering folder {:?}", root_path);

    let build_decl_file = root_path.join("build.leaf");
    let content = std::fs::read_to_string(build_decl_file).map_err(|err| {
        error!("`build.leaf' in {:?}: {}", root_path, err);
        err
    })?;
    let result = grammar::parse(&content);

    match result {
        Ok(build_definition) => {
            let fid = handle
                .get_env_mut()
                .register_new_file(root_path.to_string_lossy().to_string(), content);
            let mut frame = env::FileFrame::new(fid);
            build_definition
                .get_statements()
                .iter()
                .for_each(|statement| {
                    internal::run_statement(&mut frame, statement);
                });
        }
        Err(error) => {
            handle.get_env_mut().register_file_and_report(
                &root_path.to_string_lossy().to_string(),
                &content,
                |fid| LeafParseError::from((fid, error)),
            );
        }
    }

    Ok(())
}
