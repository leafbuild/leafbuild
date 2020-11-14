//! # Stuff related to the toolchains

use crate::buildsys_utils::toolchains::options::c::{
    CompilationOption, CompilationOptions, LinkOption, LinkOptions,
};
use crate::buildsys_utils::toolchains::options::cpp::{
    CXXCompilationOption, CXXCompilationOptions, CXXLinkOption, CXXLinkOptions,
};
use itertools::Itertools as _;
use std::env::VarError;
use std::io;
use std::path::Path;
use std::string::FromUtf8Error;

use thiserror::Error;

pub mod c;
pub mod cpp;

pub mod options;

/// An error returned when [`get_c_toolchain`][get_c_toolchain] and [`get_cpp_toolchain`][get_cpp_toolchain] couldn't figure out the toolchain used.
///
/// [get_c_toolchain]: c::get_c_toolchain
/// [get_cpp_toolchain]: cpp::get_cpp_toolchain
#[derive(Error, Debug)]
pub enum GetToolchainError {
    /// Cannot get the CC/CXX environment variable
    #[error("cannot get variable from environment")]
    VarError(#[from] VarError),
    /// Something happened while invoking `$CC --version`/`$CXX --version` and
    #[error("cannot read from compiler --version invocation")]
    ProcessError(#[from] io::Error),
    /// The `$CC --version`/`$CXX --version` sent to stdout invalid UTF-8.
    #[error("invalid utf8 in $compiler --version output")]
    InvalidUtf8(#[from] FromUtf8Error),
    /// Cannot recognize the compiler family. The recognized ones are defined at
    /// <https://leafbuild.github.io/supported_languages.html#supported-languages>
    #[error("unrecognized compiler family `{0}`")]
    UnrecognizedCompilerFamily(String),
}

/// The toolchain
pub trait Toolchain {
    /// Returns `true` if it is able to consume the given filename.
    /// For example, a typical C toolchain can consume files whose name ends with `.c`, `.h`, `.o`(`.obj` on Windows), `.tu`.
    fn can_consume(filename: &str) -> bool;
    /// Returns `true` if it is able to compile the given filename.
    /// For example, a typical C toolchain can compile only files whose name ends with `.c`.
    fn can_compile(filename: &str) -> bool;
}

/// The C toolchain
pub trait CToolchain: Toolchain {
    /// The C compiler type used.
    type Compiler: CCompiler;
    /// The linker type used.
    type Linker: CToolchainLinker;

    /// Returns the compiler this toolchain uses.
    fn get_compiler(&self) -> &Self::Compiler;

    /// Returns the linker this toolchain uses.
    fn get_linker(&self) -> &Self::Linker;
}

/// A C compiler.
pub trait CCompiler {
    /// Returns the flag as the compiler's cli expects it, from the higher-level enum for compilation flags.
    fn get_option(&self, flag: CompilationOption) -> String;

    /// Returns a string with all the flags that will be passed to the compiler.
    fn get_options(&self, options: CompilationOptions) -> String {
        options
            .into_flags_iter()
            .map(|option| self.get_option(option))
            .join(" ")
    }

    /// Returns the location of the compiler executable
    fn get_location(&self) -> &Path;
}

/// A C linker.
pub trait CToolchainLinker {
    /// Gets a string from a link option, respecting the linker's cli syntax.
    fn get_linker_option(&self, option: LinkOption) -> String;
    /// Gets a string with all options passed to the linker(from `options`)
    fn get_linker_options(&self, options: LinkOptions) -> String {
        options
            .into_flags_iter()
            .map(|option| self.get_linker_option(option))
            .join(" ")
    }

    /// Gets the path of the linker executable
    fn get_location(&self) -> &Path;
}

/// A C++ toolchain
pub trait CPPToolchain: Toolchain {
    /// The C++ compiler
    type Compiler: CPPCompiler;
    /// The linker
    type Linker: CPPToolchainLinker;

    /// Returns the compiler this toolchain uses
    fn get_compiler(&self) -> &Self::Compiler;
    /// Returns the linker this toolchain uses
    fn get_linker(&self) -> &Self::Linker;
}

/// A C++ compiler
pub trait CPPCompiler {
    /// Returns the string that should be passed to the compiler's cli from the compilation option
    fn get_option(&self, option: CXXCompilationOption) -> String;
    /// Returns a string equivalent to all the options respecting the compiler's cli
    fn get_flags(&self, options: CXXCompilationOptions) -> String {
        options
            .into_flags_iter()
            .map(|option| self.get_option(option))
            .join(" ")
    }

    /// Returns the path to the compiler executable
    fn get_location(&self) -> &Path;
}

/// A C++ toolchain linker
pub trait CPPToolchainLinker {
    /// Returns the string that should be passed to the linker's CLI from the link option
    fn get_option(&self, option: CXXLinkOption) -> String;
    /// Returns a string equivalent of all the options respecting the linker's cli
    fn get_options(&self, options: CXXLinkOptions) -> String {
        options
            .into_flags_iter()
            .map(|option| self.get_option(option))
            .join(" ")
    }

    /// Returns the path to the linker executable
    fn get_location(&self) -> &Path;
}
