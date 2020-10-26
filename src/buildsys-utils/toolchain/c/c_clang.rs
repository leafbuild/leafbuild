use crate::buildsys_utils::toolchain::flags::c::{CCompilationFlag, CFlag, CLinkFlag};
use crate::buildsys_utils::toolchain::{CCompiler, CToolchain, CToolchainLinker, Toolchain};
use std::path::{Path, PathBuf};

pub struct CClangToolchain {
    clang: CClang,
}

impl CClangToolchain {
    pub(crate) fn new(clang_location: Box<Path>) -> Self {
        Self {
            clang: CClang {
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
    type Compiler = CClang;
    type Linker = CClang;

    fn get_compiler(&self) -> &Self::Compiler {
        &self.clang
    }

    fn get_linker(&self) -> &Self::Linker {
        &self.clang
    }
}

pub struct CClang {
    path: PathBuf,
}

impl CCompiler for CClang {
    fn get_flag(&self, flag: CCompilationFlag) -> String {
        match flag {
            CCompilationFlag::FromString { s } => s,
            CCompilationFlag::CSTD { std } => format!("--std={}", std.as_str()),
            CCompilationFlag::IncludeDir { include_dir } => format!("-I{}", include_dir),
            CCompilationFlag::Flag { flag } => match flag {
                CFlag::PositionIndependentCode => "-fPIC".into(),
            },
            CCompilationFlag::None => "".into(),
        }
    }

    fn get_location(&self) -> &Path {
        self.path.as_path()
    }
}

impl CToolchainLinker for CClang {
    fn get_linker_flag(&self, flag: CLinkFlag) -> String {
        match flag {
            CLinkFlag::FromString { s } => s,
            CLinkFlag::LibLocation { s } => format!("-L{}", s),
            CLinkFlag::Lib { name } => format!("-l{}", name),
            CLinkFlag::LibShared => "--shared".into(),
            CLinkFlag::None => "".into(),
        }
    }

    fn get_location(&self) -> &Path {
        self.path.as_path()
    }
}
