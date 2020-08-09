use super::Compiler;
use crate::compilers::GetCompilerError;
use std::process::Command;

pub enum CCFamily {
    GCC,
    Clang,
}

pub struct CC {
    family: CCFamily,
}

impl Compiler for CC {
    fn can_consume(&self, filename: &str) -> bool {
        self.can_compile(filename) || filename.ends_with(".h")
    }
    fn can_compile(&self, filename: &str) -> bool {
        filename.ends_with(".c")
    }
}

pub fn get_cc() -> Result<CC, GetCompilerError> {
    let compiler_location = std::env::var("CC")?;

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
