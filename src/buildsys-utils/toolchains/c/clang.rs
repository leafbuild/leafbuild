use crate::buildsys_utils::toolchains::flags::c::{CompilationFlag, Flag, LinkFlag};
use crate::buildsys_utils::toolchains::{CCompiler, CToolchain, CToolchainLinker, Toolchain};
use std::path::{Path, PathBuf};

pub struct CClangToolchain {
    clang: Clang,
}

impl CClangToolchain {
    pub(crate) fn new(clang_location: Box<Path>) -> Self {
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

pub struct Clang {
    path: PathBuf,
}

impl CCompiler for Clang {
    fn get_flag(&self, flag: CompilationFlag) -> String {
        match flag {
            CompilationFlag::FromString { s } => s,
            CompilationFlag::CSTD { std } => format!("--std={}", std.to_string()),
            CompilationFlag::IncludeDir { include_dir } => format!("-I{}", include_dir),
            CompilationFlag::Flag { flag } => match flag {
                Flag::PositionIndependentCode => "-fPIC".into(),
            },
            CompilationFlag::None => "".into(),
        }
    }

    fn get_location(&self) -> &Path {
        self.path.as_path()
    }
}

impl CToolchainLinker for Clang {
    fn get_linker_flag(&self, flag: LinkFlag) -> String {
        match flag {
            LinkFlag::FromString { s } => s,
            LinkFlag::LibLocation { s } => format!("-L{}", s),
            LinkFlag::Lib { name } => format!("-l{}", name),
            LinkFlag::LibShared => "--shared".into(),
            LinkFlag::None => "".into(),
        }
    }

    fn get_location(&self) -> &Path {
        self.path.as_path()
    }
}
