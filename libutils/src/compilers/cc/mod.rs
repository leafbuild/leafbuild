use super::Compiler;
use crate::compilers::flags::{CompilationFlag, CompilationFlags, Flag, LinkFlag, LinkFlags, CSTD};
use crate::compilers::GetCompilerError;
use itertools::Itertools;
use std::path::PathBuf;
use std::process::Command;

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

    fn get_flag(&self, flag: CompilationFlag) -> String {
        match self.family {
            CCFamily::GCC | CCFamily::Clang => match flag {
                CompilationFlag::FromString { s } => s,
                CompilationFlag::CSTD { std } => format!(
                    "--c_std={}",
                    match std {
                        CSTD::ANSI => "ansi",
                        CSTD::C99 => "c99",
                        CSTD::GNU99 => "gnu99",
                        CSTD::C11 => "c11",
                        CSTD::GNU11 => "gnu11",
                    }
                ),
                CompilationFlag::IncludeDir { include_dir } => format!("-I {}", include_dir),
                CompilationFlag::Flag { flag } => format!(
                    "-f{}",
                    match flag {
                        Flag::PositionIndependentCode => "PIC",
                    }
                ),
                _ => "".to_string(),
            },
            CCFamily::MSVC => {
                match flag {
                    CompilationFlag::FromString { s } => s,

                    // TODO: add those later
                    _ => "".to_string(),
                }
            }
        }
    }

    fn get_linker_flag(&self, flag: LinkFlag) -> String {
        match self.family {
            CCFamily::GCC | CCFamily::Clang => match flag {
                LinkFlag::FromString { s } => s,
                LinkFlag::LibLocation { s: loc } => format!("-L {}", loc),
                LinkFlag::Lib { name } => format!("-l{}", name),
            },

            CCFamily::MSVC => match flag {
                LinkFlag::FromString { s } => s,
                _ => "".to_string(),
            },
        }
    }

    fn get_flags(&self, flags: CompilationFlags) -> String {
        flags.into_flags_iter().map(|f| self.get_flag(f)).join(" ")
    }

    fn get_linker_flags(&self, flags: LinkFlags) -> String {
        flags
            .into_flags_iter()
            .map(|f| self.get_linker_flag(f))
            .join(" ")
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
