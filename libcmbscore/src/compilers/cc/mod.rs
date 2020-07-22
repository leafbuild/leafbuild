use super::Compiler;

pub(crate) trait CCompiler {}

impl Compiler for dyn CCompiler {
    fn can_consume(filename: &str) -> bool {
        Self::can_compile(filename) || filename.ends_with(".h")
    }
    fn can_compile(filename: &str) -> bool {
        filename.ends_with(".c")
    }
}

/// The clang C compiler

struct Clang {}

impl CCompiler for Clang {}

/// The GCC C compiler

struct GCC {}

impl CCompiler for GCC {}
