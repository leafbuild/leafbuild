mod diagnostics;
pub(crate) mod env;
mod internal;

use crate::{grammar, handle::Handle};
use lalrpop_util::ParseError;
use std::{error::Error, path::PathBuf};

pub(crate) const DOCS_ROOT: &str = "https://leafbuild.gitlab.io/docs";

pub fn start_on<'a>(handle: &'a mut Handle<'a>, root_path: PathBuf) -> Result<(), Box<dyn Error>> {
    info!("Entering folder {:?}", root_path);

    let build_decl_file = root_path.join("build.leaf");
    let content = std::fs::read_to_string(build_decl_file).map_err(|err| {
        error!("`build.leaf' in {:?}: {}", root_path, err);
        err
    })?;

    match grammar::parse(&content) {
        Ok(build_definition) => {
            let fid = handle
                .get_env_mut()
                .register_new_file(root_path.to_string_lossy().to_string(), content);
            let mut frame = env::EnvFrame::new(fid);
            build_definition
                .get_statements()
                .iter()
                .for_each(|statement| {
                    internal::run_statement(&mut frame, statement);
                });
        }
        Err(error) => match error {
            ParseError::InvalidToken { location } => {
                println!("Invalid token at {}", location);
            }
            ParseError::UnrecognizedEOF { location, expected } => {
                println!("Unrecognized EOF at {}, expected {:?}", location, expected);
            }
            ParseError::UnrecognizedToken { token, expected } => {
                println!(
                    "Unrecognized token at {:?}, expected {:?}, found {:?}",
                    token.0..token.2,
                    expected,
                    token.1,
                );
            }
            ParseError::ExtraToken { .. } => {}
            ParseError::User { .. } => {}
        },
    }

    Ok(())
}
