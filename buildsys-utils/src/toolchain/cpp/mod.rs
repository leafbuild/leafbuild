pub mod cpp_clang;
pub mod cpp_gcc;

use crate::toolchain::{CPPCompiler, CPPToolchain, CPPToolchainLinker, GetToolchainError};
use cpp_clang::CPPClangToolchain;

use crate::toolchain::cpp::cpp_clang::CPPClang;
use std::path::{Path, PathBuf};
use std::process::Command;

pub enum CPPTc {
    CPPGcc,
    CPPClang(CPPClangToolchain),
}

impl CPPTc {
    pub fn get_compiler_location(&self) -> &Path {
        match self {
            CPPTc::CPPGcc => unimplemented!(),
            CPPTc::CPPClang(clang) => <CPPClang as CPPCompiler>::get_location(clang.get_compiler()),
        }
    }

    pub fn get_linker_location(&self) -> &Path {
        match self {
            CPPTc::CPPGcc => unimplemented!(),
            CPPTc::CPPClang(clang) => {
                <CPPClang as CPPToolchainLinker>::get_location(clang.get_linker())
            }
        }
    }
}

pub fn get_cpp_toolchain() -> Result<CPPTc, GetToolchainError> {
    let compiler_location = match std::env::var("CXX") {
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
        // family if family.contains("(GCC)") => Ok(CTc::CGcc(location)),
        family if family.contains("clang") => Ok(CPPTc::CPPClang(CPPClangToolchain::new(
            location.into_boxed_path(),
        ))),
        family => Err(GetToolchainError::UnrecognizedCompilerFamily(
            family.to_string(),
        )),
    }
}
