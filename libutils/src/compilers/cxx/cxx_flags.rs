use crate::compilers::flags::{CompilationFlag, CompilationFlags, LinkFlag, LinkFlags};

pub enum CPPSTD {
    CPP98,
    CPP03,

    // C++11 = CPP1x
    CPP1x,

    // C++14 = CPP1y
    CPP1y,

    // C++17 = CPP1z
    CPP1z,

    // C++20 = CPP2a
    CPP2a,
}

pub enum CXXFlag {
    FromString { string: String },
    CPPSTD { std: CPPSTD },
    IncludeDir { include_dir: String },
}

impl CompilationFlag for CXXFlag {
    fn from_string(s: impl Into<String>) -> Self {
        Self::FromString { string: s.into() }
    }
}

pub enum CXXLDFlag {
    FromString { string: String },
}

impl LinkFlag for CXXLDFlag {
    fn from_string(s: impl Into<String>) -> Self {
        Self::FromString { string: s.into() }
    }
}

pub type CXXFlags = CompilationFlags<CXXFlag>;
pub type CXXLDFlags = LinkFlags<CXXLDFlag>;
