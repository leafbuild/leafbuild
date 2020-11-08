pub enum STD {
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

pub enum CXXFlag {
    PositionIndependentCode,
}

pub enum CXXCompilationFlag {
    FromString { s: String },
    CPPSTD { std: STD },
    IncludeDir { include_dir: String },

    Flag { flag: CXXFlag },

    None,
}

impl CXXCompilationFlag {
    /// creates a flag from a string
    #[must_use]
    pub fn from_string(s: impl Into<String>) -> Self {
        Self::FromString { s: s.into() }
    }
}

pub enum CXXLinkFlag {
    FromString { s: String },
    LibLocation { s: String },
    Lib { name: String },
    LibShared,
    None,
}

impl CXXLinkFlag {
    /// creates a flag from a string
    #[must_use]
    pub fn from_string(s: impl Into<String>) -> Self {
        Self::FromString { s: s.into() }
    }
}

pub struct CXXCompilationFlags {
    flags: Vec<CXXCompilationFlag>,
}

impl CXXCompilationFlags {
    #[must_use]
    pub const fn empty() -> Self {
        Self::new(vec![])
    }

    #[must_use]
    pub const fn new(flags: Vec<CXXCompilationFlag>) -> Self {
        Self { flags }
    }

    pub(crate) fn into_flags_iter(self) -> impl Iterator<Item = CXXCompilationFlag> {
        self.flags.into_iter()
    }
}

pub struct CXXLinkFlags {
    flags: Vec<CXXLinkFlag>,
}

impl CXXLinkFlags {
    #[must_use]
    pub const fn empty() -> Self {
        Self::new(vec![])
    }

    #[must_use]
    pub const fn new(flags: Vec<CXXLinkFlag>) -> Self {
        Self { flags }
    }

    pub(crate) fn into_flags_iter(self) -> impl Iterator<Item = CXXLinkFlag> {
        self.flags.into_iter()
    }
}
