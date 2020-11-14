//! The C++ Clang toolchain
use crate::buildsys_utils::toolchains::options::cpp::{
    CXXCompilationOption, CXXFlag, CXXLinkOption,
};
use crate::buildsys_utils::toolchains::{CPPCompiler, CPPToolchain, CPPToolchainLinker, Toolchain};
use std::path::{Path, PathBuf};

/// The C++ Clang toolchain structure
pub struct CPPClangToolchain {
    clang: Clang,
}

impl CPPClangToolchain {
    pub(crate) fn new(clang_location: Box<Path>) -> Self {
        Self {
            clang: Clang {
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
pub struct Clang {
    location: PathBuf,
}

impl CPPCompiler for Clang {
    fn get_option(&self, flag: CXXCompilationOption) -> String {
        match flag {
            CXXCompilationOption::FromString(s) => s,
            CXXCompilationOption::CPPSTD(std) => format!("--std={}", std.to_string()),
            CXXCompilationOption::IncludeDir(include_dir) => format!("-I{}", include_dir),
            CXXCompilationOption::Flag(flag) => match flag {
                CXXFlag::PositionIndependentCode => "-fPIC".into(),
            },
            CXXCompilationOption::None => "".into(),
        }
    }

    fn get_location(&self) -> &Path {
        self.location.as_path()
    }
}

impl CPPToolchainLinker for Clang {
    fn get_option(&self, _flag: CXXLinkOption) -> String {
        unimplemented!()
    }

    fn get_location(&self) -> &Path {
        self.location.as_path()
    }
}
