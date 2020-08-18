use super::{Compiler, GetCompilerError};
use std::process::Command;

use crate::compilers::flags::{
    CompilationFlag, CompilationFlags, Flag, LinkFlag, LinkFlags, CPPSTD,
};
use itertools::Itertools;
use std::path::PathBuf;

#[derive(Copy, Clone)]
pub enum CXXFamily {
    GCC,
    Clang,
    MSVC,
}

#[derive(Clone)]
pub struct CXX {
    family: CXXFamily,
    location: PathBuf,
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

    fn get_location(&self) -> &PathBuf {
        &self.location
    }

    fn get_flag(&self, flag: CompilationFlag) -> String {
        match self.family {
            CXXFamily::GCC | CXXFamily::Clang => match flag {
                CompilationFlag::FromString { s } => s,
                CompilationFlag::CPPSTD { std } => format!(
                    "--cpp_std={}",
                    match std {
                        CPPSTD::CPP98 => "c++98",
                        CPPSTD::CPP03 => "c++03",
                        CPPSTD::CPP1x => "c++1x",
                        CPPSTD::CPP1y => "c++1y",
                        CPPSTD::CPP1z => "c++1z",
                        CPPSTD::CPP2a => "c++2a",
                    }
                ),
                CompilationFlag::IncludeDir { include_dir } => format!("-I {}", include_dir),
                CompilationFlag::Flag { flag } => format!(
                    "-f{}",
                    match flag {
                        Flag::PositionIndependentCode => "PIC",
                    }
                ),
                CompilationFlag::None => "".to_string(),
                _ => "".to_string(),
            },
            CXXFamily::MSVC => {
                // TODO add this later
                "".to_string()
            }
        }
    }

    fn get_linker_flag(&self, flag: LinkFlag) -> String {
        match self.family {
            CXXFamily::GCC | CXXFamily::Clang => match flag {
                LinkFlag::FromString { s } => s,
                LinkFlag::LibLocation { s: loc } => format!("-L {}", loc),
                LinkFlag::Lib { name } => format!("-l{}", name),
                LinkFlag::LibShared => "-shared".to_string(),
                LinkFlag::None => "".to_string(),
            },

            CXXFamily::MSVC => match flag {
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

    let location = compiler_location.clone();

    let output = Command::new(compiler_location).arg("--version").output()?;
    let output = String::from_utf8(output.stdout)?;
    let first_line = output
        .lines()
        .next() // get first line
        .expect("Cannot detect compiler family from `CXX --version'");

    match first_line {
        family if family.contains("(GCC)") => Ok(CXX {
            family: CXXFamily::GCC,
            location,
        }),
        family if family.contains("clang") => Ok(CXX {
            family: CXXFamily::Clang,
            location,
        }),
        family => Err(GetCompilerError::UnrecognizedCompilerFamily(
            family.to_string(),
        )),
    }
}
