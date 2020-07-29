use super::Compiler;

pub(crate) trait CXXCompiler {}

impl Compiler for dyn CXXCompiler {
    fn can_consume(filename: &str) -> bool {
        Self::can_compile(filename)
            || filename.ends_with(".h")
            || filename.ends_with(".hpp")
            || filename.ends_with(".hh")
            || filename.ends_with(".hxx")
    }

    fn can_compile(filename: &str) -> bool {
        filename.ends_with(".cpp") || filename.ends_with(".cc") || filename.ends_with(".cxx")
    }
}

/// The clang compiler

struct Clang {}

impl CXXCompiler for Clang {}

/// The GCC c++ compiler

struct GCC {}

impl CXXCompiler for GCC {}
