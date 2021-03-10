//! # The Clang C toolchain.
//!
use crate::buildsys_utils::toolchains::options::c::{CompilationOption, Flag, LinkOption};
use crate::buildsys_utils::toolchains::{CCompiler, CToolchain, CToolchainLinker, Toolchain};
use std::path::{Path, PathBuf};

/// The struct. See the module-level docs for more.
#[derive(Debug)]
pub struct CClangToolchain {
    clang: Clang,
}

impl CClangToolchain {
    /// Creates a new instance from the location of the clang executable.
    #[must_use]
    pub fn new(clang_location: Box<Path>) -> Self {
        Self {
            clang: Clang {
                path: clang_location.into_path_buf(),
            },
        }
    }
}

impl Toolchain for CClangToolchain {
    fn can_consume(filename: &str) -> bool {
        filename.ends_with(".c") || filename.ends_with(".h")
    }

    fn can_compile(filename: &str) -> bool {
        filename.ends_with(".c")
    }
}

impl CToolchain for CClangToolchain {
    type Compiler = Clang;
    type Linker = Clang;

    fn get_compiler(&self) -> &Self::Compiler {
        &self.clang
    }

    fn get_linker(&self) -> &Self::Linker {
        &self.clang
    }
}

/// The clang compiler and linker
#[derive(Debug)]
pub struct Clang {
    path: PathBuf,
}

impl CCompiler for Clang {
    fn get_option(&self, flag: CompilationOption) -> String {
        match flag {
            CompilationOption::FromString(s) => s,
            CompilationOption::CSTD(std) => format!("--std={}", std.to_string()),
            CompilationOption::IncludeDir(include_dir) => format!("-I{}", include_dir),
            CompilationOption::Flag(flag) => match flag {
                Flag::PositionIndependentCode => "-fPIC".into(),
            },
            CompilationOption::None => "".into(),
        }
    }

    fn get_location(&self) -> &Path {
        self.path.as_path()
    }
}

impl CToolchainLinker for Clang {
    fn get_linker_option(&self, flag: LinkOption) -> String {
        match flag {
            LinkOption::FromString(s) => s,
            LinkOption::LibLocation(s) => format!("-L{}", s),
            LinkOption::Lib { name } => format!("-l{}", name),
            LinkOption::LibShared => "--shared".into(),
            LinkOption::None => "".into(),
        }
    }

    fn get_location(&self) -> &Path {
        self.path.as_path()
    }
}
