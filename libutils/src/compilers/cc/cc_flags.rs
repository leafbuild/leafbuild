use crate::compilers::flags::{CompilationFlag, CompilationFlags, LinkFlag, LinkFlags};

pub enum CSTD {
    // ANSI = C89 = C90
    ANSI,

    C99,
    GNU99,

    C11,
    GNU11,
    // TODO: add more here as they come
}

pub enum CCFlag {
    FromString { string: String },
    CSTD { std: CSTD },
    IncludeDir { include_dir: String },
}

impl CompilationFlag for CCFlag {
    fn from_string(s: impl Into<String>) -> Self {
        Self::FromString { string: s.into() }
    }
}

pub enum CCLDFlag {
    FromString { string: String },
}

impl LinkFlag for CCLDFlag {
    fn from_string(s: impl Into<String>) -> Self {
        Self::FromString { string: s.into() }
    }
}

pub type CCFlags = CompilationFlags<CCFlag>;
pub type CCLDFlags = LinkFlags<CCLDFlag>;
