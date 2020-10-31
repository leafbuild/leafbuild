use crate::buildsys_utils::toolchain::flags::cpp::{CXXCompilationFlag, CXXFlag, CXXLinkFlag};
use crate::buildsys_utils::toolchain::{CPPCompiler, CPPToolchain, CPPToolchainLinker, Toolchain};
use std::path::{Path, PathBuf};

pub struct CPPClangToolchain {
    clang: CPPClang,
}

impl CPPClangToolchain {
    pub(crate) fn new(clang_location: Box<Path>) -> Self {
        Self {
            clang: CPPClang {
                location: clang_location.into_path_buf(),
            },
        }
    }
}

impl Toolchain for CPPClangToolchain {
    fn can_consume(filename: &str) -> bool {
        Self::can_compile(filename)
            || filename.ends_with(".h")
            || filename.ends_with(".hpp")
            || filename.ends_with(".hxx")
            || filename.ends_with(".h++")
    }

    fn can_compile(filename: &str) -> bool {
        filename.ends_with(".c")
            || filename.ends_with(".cpp")
            || filename.ends_with(".c++")
            || filename.ends_with(".cxx")
    }
}

impl CPPToolchain for CPPClangToolchain {
    type Compiler = CPPClang;
    type Linker = CPPClang;

    fn get_compiler(&self) -> &Self::Compiler {
        &self.clang
    }

    fn get_linker(&self) -> &Self::Linker {
        &self.clang
    }
}

pub struct CPPClang {
    location: PathBuf,
}

impl CPPCompiler for CPPClang {
    fn get_flag(&self, flag: CXXCompilationFlag) -> String {
        match flag {
            CXXCompilationFlag::FromString { s } => s,
            CXXCompilationFlag::CPPSTD { std } => format!("--std={}", std.to_string()),
            CXXCompilationFlag::IncludeDir { include_dir } => format!("-I{}", include_dir),
            CXXCompilationFlag::Flag { flag } => match flag {
                CXXFlag::PositionIndependentCode => "-fPIC".into(),
            },
            CXXCompilationFlag::None => "".into(),
        }
    }

    fn get_location(&self) -> &Path {
        self.location.as_path()
    }
}

impl CPPToolchainLinker for CPPClang {
    fn get_flag(&self, _flag: CXXLinkFlag) -> String {
        unimplemented!()
    }

    fn get_location(&self) -> &Path {
        self.location.as_path()
    }
}
