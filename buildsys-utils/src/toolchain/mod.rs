//! # Stuff related to the toolchains

use crate::toolchain::flags::c::{CCompilationFlag, CCompilationFlags, CLinkFlag, CLinkFlags};
use crate::toolchain::flags::cpp::{
    CXXCompilationFlag, CXXCompilationFlags, CXXLinkFlag, CXXLinkFlags,
};
use itertools::Itertools as _;
use std::env::VarError;
use std::io;
use std::path::Path;
use std::string::FromUtf8Error;

pub mod c;
pub mod cpp;

pub mod flags;

#[derive(Debug)]
pub enum GetToolchainError {
    VarError(VarError),
    ProcessError(io::Error),
    InvalidUtf8(FromUtf8Error),
    UnrecognizedCompilerFamily(String),
}

impl From<VarError> for GetToolchainError {
    #[inline]
    fn from(v: VarError) -> Self {
        GetToolchainError::VarError(v)
    }
}

impl From<io::Error> for GetToolchainError {
    #[inline]
    fn from(v: io::Error) -> Self {
        GetToolchainError::ProcessError(v)
    }
}

impl From<FromUtf8Error> for GetToolchainError {
    #[inline]
    fn from(v: FromUtf8Error) -> Self {
        GetToolchainError::InvalidUtf8(v)
    }
}

pub trait Toolchain {
    fn can_consume(filename: &str) -> bool;
    fn can_compile(filename: &str) -> bool;
}

pub trait CToolchain: Toolchain {
    type Compiler: CCompiler;
    type Linker: CToolchainLinker;

    fn get_compiler(&self) -> &Self::Compiler;
    fn get_linker(&self) -> &Self::Linker;
}

pub trait CCompiler {
    fn get_flag(&self, flag: CCompilationFlag) -> String;
    fn get_flags(&self, flags: CCompilationFlags) -> String {
        flags
            .into_flags_iter()
            .map(|flag| self.get_flag(flag))
            .join(" ")
    }

    fn get_location(&self) -> &Path;
}

pub trait CToolchainLinker {
    fn get_linker_flag(&self, flag: CLinkFlag) -> String;
    fn get_linker_flags(&self, flags: CLinkFlags) -> String {
        flags
            .into_flags_iter()
            .map(|flag| self.get_linker_flag(flag))
            .join(" ")
    }

    fn get_location(&self) -> &Path;
}

pub trait CPPToolchain: Toolchain {
    type Compiler: CPPCompiler;
    type Linker: CPPToolchainLinker;

    fn get_compiler(&self) -> &Self::Compiler;
    fn get_linker(&self) -> &Self::Linker;
}

pub trait CPPCompiler {
    fn get_flag(&self, flag: CXXCompilationFlag) -> String;
    fn get_flags(&self, flags: CXXCompilationFlags) -> String {
        flags
            .into_flags_iter()
            .map(|flag| self.get_flag(flag))
            .join(" ")
    }

    fn get_location(&self) -> &Path;
}

pub trait CPPToolchainLinker {
    fn get_flag(&self, flag: CXXLinkFlag) -> String;
    fn get_flags(&self, flags: CXXLinkFlags) -> String {
        flags
            .into_flags_iter()
            .map(|flag| self.get_flag(flag))
            .join(" ")
    }

    fn get_location(&self) -> &Path;
}
