//! Related to the C toolchains
pub mod clang;
pub mod gcc;

use crate::buildsys_utils::toolchains::{
    CCompiler, CToolchain, CToolchainLinker, GetToolchainError,
};
use clang::CClangToolchain;
use std::process::Command;

use crate::buildsys_utils::toolchains::c::clang::Clang;
use std::path::{Path, PathBuf};

/// Stands for Toolchain; an enum to store all possible values because the [`CToolchain`] trait is not object-safe.
pub enum Tc {
    /// The Gcc C toolchain (gcc + ld)
    Gcc,
    /// The Clang C toolchain
    Clang(CClangToolchain),
}

impl Tc {
    /// Returns the location of the compiler.
    #[must_use]
    pub fn get_compiler_location(&self) -> &Path {
        match self {
            Self::Gcc => unimplemented!(),
            Self::Clang(clang) => <Clang as CCompiler>::get_location(clang.get_compiler()),
        }
    }

    /// Returns the location of the linker.
    #[must_use]
    pub fn get_linker_location(&self) -> &Path {
        match self {
            Self::Gcc => unimplemented!(),
            Self::Clang(clang) => <Clang as CToolchainLinker>::get_location(clang.get_linker()),
        }
    }
}

/// Gets the C toolchain which is selected with the `CC` environment variable
/// # Errors
/// Environment variable not present / couldn't be parsed.
/// Output of "$CC" --version in an unexpected format
pub fn get_c_toolchain() -> Result<Tc, GetToolchainError> {
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
        .ok_or_else(|| {
            GetToolchainError::UnrecognizedCompilerFamily(
                "no lines in output of `$CC --version`".into(),
            )
        })?;

    match first_line {
        // family if family.contains("(GCC)") => Ok(CTc::CGcc(location)),
        family if family.contains("clang") => {
            Ok(Tc::Clang(CClangToolchain::new(location.into_boxed_path())))
        }
        family => Err(GetToolchainError::UnrecognizedCompilerFamily(
            family.to_string(),
        )),
    }
}
