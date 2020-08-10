use super::Compiler;
use crate::compilers::GetCompilerError;
use std::path::PathBuf;
use std::process::Command;

pub enum CCFamily {
    GCC,
    Clang,
}

pub struct CC {
    family: CCFamily,
}

impl Compiler for CC {
    fn can_consume(filename: &str) -> bool {
        Self::can_compile(filename) || filename.ends_with(".h")
    }
    fn can_compile(filename: &str) -> bool {
        filename.ends_with(".c")
    }
}

pub fn get_cc() -> Result<CC, GetCompilerError> {
    let compiler_location = match std::env::var("CC") {
        Ok(p) => Ok(PathBuf::from(p)),
        Err(err) => {
            if cfg!(target_os = "linux") {
                Ok(PathBuf::from("/usr/bin/cc"))
            } else {
                Err(err)
            }
        }
    }?;

    let output = Command::new(compiler_location).arg("--version").output()?;

    match String::from_utf8(output.stdout)?
        .lines()
        .next() // get first line
        .expect("Cannot detect compiler family from `CC --version'")
        .split_ascii_whitespace()
        .next() // first fragment
        .expect("Cannot detect compiler family from `CC --version'")
    {
        "gcc" => Ok(CC {
            family: CCFamily::GCC,
        }),
        "clang" => Ok(CC {
            family: CCFamily::Clang,
        }),
        family => Err(GetCompilerError::UnrecognizedCompilerFamily(
            family.to_string(),
        )),
    }
}
