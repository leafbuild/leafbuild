pub mod clang;
pub mod gcc;

use crate::buildsys_utils::toolchains::{
    CPPCompiler, CPPToolchain, CPPToolchainLinker, GetToolchainError,
};
use clang::CPPClangToolchain;

use crate::buildsys_utils::toolchains::cpp::clang::Clang;
use std::path::{Path, PathBuf};
use std::process::Command;

pub enum Tc {
    CPPGcc,
    CPPClang(CPPClangToolchain),
}

impl Tc {
    #[must_use]
    pub fn get_compiler_location(&self) -> &Path {
        match self {
            Self::CPPGcc => unimplemented!(),
            Self::CPPClang(clang) => <Clang as CPPCompiler>::get_location(clang.get_compiler()),
        }
    }

    #[must_use]
    pub fn get_linker_location(&self) -> &Path {
        match self {
            Self::CPPGcc => unimplemented!(),
            Self::CPPClang(clang) => {
                <Clang as CPPToolchainLinker>::get_location(clang.get_linker())
            }
        }
    }
}

/// Gets the C++ toolchain which is selected with the `CXX` environment variable
/// # Errors
/// Environment variable not present / couldn't be parsed.
/// Output of "$CXX" --version in an unexpected format
pub fn get_cpp_toolchain() -> Result<Tc, GetToolchainError> {
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
        family if family.contains("clang") => Ok(Tc::CPPClang(CPPClangToolchain::new(
            location.into_boxed_path(),
        ))),
        family => Err(GetToolchainError::UnrecognizedCompilerFamily(
            family.to_string(),
        )),
    }
}
