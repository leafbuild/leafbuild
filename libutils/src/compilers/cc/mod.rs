use super::Compiler;
use crate::compilers::GetCompilerError;
use std::path::PathBuf;
use std::process::Command;

pub mod cc_flags;
use crate::compilers::cc::cc_flags::CSTD;
pub use cc_flags::{CCFlag, CCFlags, CCLDFlag, CCLDFlags};

#[derive(Copy, Clone)]
pub enum CCFamily {
    GCC,
    Clang,
    MSVC,
}

#[derive(Clone)]
pub struct CC {
    family: CCFamily,
    location: PathBuf,
}

impl CC {
    pub fn get_flag(&self, flag: CCFlag) -> String {
        match self.family {
            CCFamily::GCC | CCFamily::Clang => match flag {
                CCFlag::FromString { string } => string,
                CCFlag::CSTD { std } => format!(
                    "--c_std={}",
                    match std {
                        CSTD::ANSI => "ansi",
                        CSTD::C99 => "c99",
                        CSTD::GNU99 => "gnu99",
                        CSTD::C11 => "c11",
                        CSTD::GNU11 => "gnu11",
                    }
                ),
                CCFlag::IncludeDir { include_dir } => format!("-I {}", include_dir),
            },
            CCFamily::MSVC => {
                // TODO: add those later
                "".to_string()
            }
        }
    }
}

impl Compiler for CC {
    fn can_consume(filename: &str) -> bool {
        Self::can_compile(filename) || filename.ends_with(".h")
    }
    fn can_compile(filename: &str) -> bool {
        filename.ends_with(".c")
    }

    fn get_location(&self) -> &PathBuf {
        &self.location
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

    let location = compiler_location.clone();

    let output = Command::new(compiler_location).arg("--version").output()?;
    let output = String::from_utf8(output.stdout)?;
    let first_line = output
        .lines()
        .next() // get first line
        .expect("Cannot detect compiler family from `CC --version'");

    match first_line {
        family if family.contains("(GCC)") => Ok(CC {
            family: CCFamily::GCC,
            location,
        }),
        family if family.contains("clang") => Ok(CC {
            family: CCFamily::Clang,
            location,
        }),
        family => Err(GetCompilerError::UnrecognizedCompilerFamily(
            family.to_string(),
        )),
    }
}
