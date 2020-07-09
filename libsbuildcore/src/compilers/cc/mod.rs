use super::Compiler;

pub(crate) trait CCompiler {}

impl Compiler for dyn CCompiler {
    fn can_consume(filename: &String) -> bool { Self::can_compile(filename) || filename.ends_with(".h") }
    fn can_compile(filename: &String) -> bool { filename.ends_with(".c") }
}

/// The clang C compiler

struct Clang {}

impl CCompiler for Clang {}

/// The GCC C compiler

struct GCC {}

impl CCompiler for GCC {}

/// The MSVC C compiler
/// Available only in windows
#[cfg(target_os = "windows")]
struct MSVC {}

#[cfg(target_os = "windows")]
impl CCompiler for MSVC {}