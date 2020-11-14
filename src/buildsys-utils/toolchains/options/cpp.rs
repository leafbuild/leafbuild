//! C++ flags

use std::str::FromStr;

/// C++ standard
pub enum STD {
    /// C++98
    CPP98,
    /// C++03
    CPP03,

    /// C++11
    CPP1x,

    /// C++14
    CPP1y,

    /// C++17
    CPP1z,

    /// C++20
    CPP2a,
}

impl ToString for STD {
    #[must_use]
    fn to_string(&self) -> String {
        match self {
            Self::CPP98 => "c++98",
            Self::CPP03 => "c++03",
            Self::CPP1x => "c++1x",
            Self::CPP1y => "c++1y",
            Self::CPP1z => "c++1z",
            Self::CPP2a => "c++2a",
        }
        .into()
    }
}

/// Compilation flag
pub enum CXXFlag {
    /// Compile with position independent code (-fPIC in gcc/clang)
    PositionIndependentCode,
}

/// C++ Compilation flag
pub enum CXXCompilationOption {
    /// From string
    FromString(String),
    /// A C++ standard
    CPPSTD(STD),
    /// A include directory
    IncludeDir(String),

    /// A flag
    Flag(CXXFlag),

    /// Nothing
    None,
}

impl FromStr for CXXCompilationOption {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(Self::FromString(s.into()))
    }
}

/// A C++ toolchain linker option
pub enum CXXLinkOption {
    /// From string
    FromString(String),
    /// Library location
    LibLocation(String),
    /// Link with library
    Lib(String),
    /// Produce a shared library
    LibShared,
    /// Nothing
    None,
}

impl FromStr for CXXLinkOption {
    type Err = ();
    /// creates a flag from a string
    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(Self::FromString(s.into()))
    }
}

/// Multiple compilation options passed to the C++ compiler
pub struct CXXCompilationOptions {
    options: Vec<CXXCompilationOption>,
}

impl CXXCompilationOptions {
    /// Constructor with an empty vector
    #[must_use]
    pub const fn empty() -> Self {
        Self::new(vec![])
    }

    /// Constructor
    #[must_use]
    pub const fn new(options: Vec<CXXCompilationOption>) -> Self {
        Self { options }
    }

    pub(crate) fn into_flags_iter(self) -> impl Iterator<Item = CXXCompilationOption> {
        self.options.into_iter()
    }
}

/// Multiple link options passed to the linker in a C++ toolchain
pub struct CXXLinkOptions {
    options: Vec<CXXLinkOption>,
}

impl CXXLinkOptions {
    /// Constructor with empty vector
    #[must_use]
    pub const fn empty() -> Self {
        Self::new(vec![])
    }

    /// Constructor
    #[must_use]
    pub const fn new(options: Vec<CXXLinkOption>) -> Self {
        Self { options }
    }

    pub(crate) fn into_flags_iter(self) -> impl Iterator<Item = CXXLinkOption> {
        self.options.into_iter()
    }
}
