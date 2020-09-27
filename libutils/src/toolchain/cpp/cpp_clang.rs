use crate::toolchain::flags::cpp::{CXXCompilationFlag, CXXLinkFlag};
use crate::toolchain::{CPPCompiler, CPPToolchain, CPPToolchainLinker, Toolchain};
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
        unimplemented!()
    }

    fn can_compile(filename: &str) -> bool {
        unimplemented!()
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
        unimplemented!()
    }

    fn get_location(&self) -> &Path {
        self.location.as_path()
    }
}

impl CPPToolchainLinker for CPPClang {
    fn get_flag(&self, flag: CXXLinkFlag) -> String {
        unimplemented!()
    }

    fn get_location(&self) -> &Path {
        self.location.as_path()
    }
}
