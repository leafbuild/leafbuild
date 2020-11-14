//! C flags

use std::str::FromStr;

/// A C standard
pub enum STD {
    /// ANSI (= C89 = C90)
    ANSI,

    /// C99
    C99,
    /// GNU99
    GNU99,

    /// C11
    C11,
    /// GNU11
    GNU11,
    // TODO: add more here as they come
}

impl ToString for STD {
    fn to_string(&self) -> String {
        match self {
            Self::ANSI => "ansi",
            Self::C99 => "c99",
            Self::GNU99 => "gnu99",
            Self::C11 => "c11",
            Self::GNU11 => "gnu11",
        }
        .into()
    }
}

/// Flags
pub enum Flag {
    /// Position independent code (-fPIC in GCC/clang)
    PositionIndependentCode,
}

/// A compilation option
pub enum CompilationOption {
    /// From a raw string
    FromString(String),

    /// Specify C standard
    CSTD(STD),

    /// Add this directory to the list of directories to include.
    IncludeDir(String),

    /// A flag
    Flag(Flag),

    /// None(for convenience)
    None,
}

impl FromStr for CompilationOption {
    type Err = ();
    /// creates a flag from a string
    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(Self::FromString(s.into()))
    }
}

/// A link flag
pub enum LinkOption {
    /// From a raw string
    FromString(String),
    /// A library location
    LibLocation(String),
    /// Link with this library
    Lib {
        /// The name of the library to link with
        name: String,
    },
    /// Add `-shared` flag, to output a shared library.
    LibShared,
    /// None(for convenience)
    None,
}

impl FromStr for LinkOption {
    type Err = ();
    /// creates a flag from a string
    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(Self::FromString(s.into()))
    }
}

/// Compilation options
pub struct CompilationOptions {
    options: Vec<CompilationOption>,
}

impl CompilationOptions {
    /// constructor with no options
    #[must_use]
    pub const fn empty() -> Self {
        Self::new(vec![])
    }
    /// constructor
    #[must_use]
    pub const fn new(options: Vec<CompilationOption>) -> Self {
        Self { options }
    }

    pub(crate) fn into_flags_iter(self) -> impl Iterator<Item = CompilationOption> {
        self.options.into_iter()
    }
}

/// Link flags
pub struct LinkOptions {
    options: Vec<LinkOption>,
}

impl LinkOptions {
    /// constructor with no options
    #[must_use]
    pub const fn empty() -> Self {
        Self::new(vec![])
    }
    /// constructor
    #[must_use]
    pub const fn new(options: Vec<LinkOption>) -> Self {
        Self { options }
    }

    pub(crate) fn into_flags_iter(self) -> impl Iterator<Item = LinkOption> {
        self.options.into_iter()
    }
}
