//! # Stuff related to the toolchains

use crate::buildsys_utils::toolchains::flags::c::{
    CompilationFlag, CompilationFlags, LinkFlag, LinkFlags,
};
use crate::buildsys_utils::toolchains::flags::cpp::{
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
        Self::VarError(v)
    }
}

impl From<io::Error> for GetToolchainError {
    #[inline]
    fn from(v: io::Error) -> Self {
        Self::ProcessError(v)
    }
}

impl From<FromUtf8Error> for GetToolchainError {
    #[inline]
    fn from(v: FromUtf8Error) -> Self {
        Self::InvalidUtf8(v)
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
    fn get_flag(&self, flag: CompilationFlag) -> String;
    fn get_flags(&self, flags: CompilationFlags) -> String {
        flags
            .into_flags_iter()
            .map(|flag| self.get_flag(flag))
            .join(" ")
    }

    fn get_location(&self) -> &Path;
}

pub trait CToolchainLinker {
    fn get_linker_flag(&self, flag: LinkFlag) -> String;
    fn get_linker_flags(&self, flags: LinkFlags) -> String {
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
