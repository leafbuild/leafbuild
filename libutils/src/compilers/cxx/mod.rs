use super::{Compiler, GetCompilerError};
use std::process::Command;

use std::path::PathBuf;

pub enum CXXFamily {
    GCC,
    Clang,
}

pub struct CXX {
    family: CXXFamily,
}

impl Compiler for CXX {
    fn can_consume(filename: &str) -> bool {
        Self::can_compile(filename)
            || filename.ends_with(".h")
            || filename.ends_with(".hpp")
            || filename.ends_with(".hh")
            || filename.ends_with(".hxx")
    }

    fn can_compile(filename: &str) -> bool {
        filename.ends_with(".cpp")
            || filename.ends_with(".cc")
            || filename.ends_with(".cxx")
            || filename.ends_with(".c")
    }
}

pub fn get_cxx() -> Result<CXX, GetCompilerError> {
    let compiler_location = match std::env::var("CXX") {
        Ok(p) => Ok(PathBuf::from(p)),
        Err(err) => {
            if cfg!(target_os = "linux") {
                Ok(PathBuf::from("/usr/bin/c++"))
            } else {
                Err(err)
            }
        }
    }?;

    let output = Command::new(compiler_location).arg("--version").output()?;

    match String::from_utf8(output.stdout)?
        .lines()
        .next() // get first line
        .expect("Cannot detect compiler family from `CXX --version'")
        .split_ascii_whitespace()
        .next() // first fragment
        .expect("Cannot detect compiler family from `CXX --version'")
    {
        "gcc" => Ok(CXX {
            family: CXXFamily::GCC,
        }),
        "clang" => Ok(CXX {
            family: CXXFamily::Clang,
        }),
        family => Err(GetCompilerError::UnrecognizedCompilerFamily(
            family.to_string(),
        )),
    }
}
