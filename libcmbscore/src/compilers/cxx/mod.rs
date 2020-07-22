use super::Compiler;

pub(crate) trait CXXCompiler {}

impl Compiler for dyn CXXCompiler {
    fn can_consume(filename: &str) -> bool {
        Self::can_compile(filename)
            || filename.ends_with(".h")
            || filename.ends_with(".hpp")
            || filename.ends_with(".hh")
    }

    fn can_compile(filename: &str) -> bool {
        filename.ends_with(".cpp") || filename.ends_with(".cc")
    }
}

/// The clang compiler

struct Clang {}

impl CXXCompiler for Clang {}

/// The GCC c++ compiler

struct GCC {}

impl CXXCompiler for GCC {}
