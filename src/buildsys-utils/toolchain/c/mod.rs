pub mod c_clang;
pub mod c_gcc;

use crate::buildsys_utils::toolchain::{
    CCompiler, CToolchain, CToolchainLinker, GetToolchainError,
};
use c_clang::CClangToolchain;
use std::process::Command;

use crate::buildsys_utils::toolchain::c::c_clang::CClang;
use std::path::{Path, PathBuf};

/// Stands for C Toolchain; an enum to store all possible values because the [`crate::toolchain::CToolchain`] trait is not object-safe.
pub enum CTc {
    CGcc,
    CClang(CClangToolchain),
}

impl CTc {
    pub fn get_compiler_location(&self) -> &Path {
        match self {
            CTc::CGcc => unimplemented!(),
            CTc::CClang(clang) => <CClang as CCompiler>::get_location(clang.get_compiler()),
        }
    }

    pub fn get_linker_location(&self) -> &Path {
        match self {
            CTc::CGcc => unimplemented!(),
            CTc::CClang(clang) => <CClang as CToolchainLinker>::get_location(clang.get_linker()),
        }
    }
}

pub fn get_c_toolchain() -> Result<CTc, GetToolchainError> {
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
        // family if family.contains("(GCC)") => Ok(CTc::CGcc(location)),
        family if family.contains("clang") => Ok(CTc::CClang(CClangToolchain::new(
            location.into_boxed_path(),
        ))),
        family => Err(GetToolchainError::UnrecognizedCompilerFamily(
            family.to_string(),
        )),
    }
}
