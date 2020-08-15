pub enum CSTD {
    // ANSI = C89 = C90
    ANSI,

    C99,
    GNU99,

    C11,
    GNU11,
    // TODO: add more here as they come
}

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

pub enum Flag {
    PositionIndependentCode,
}

pub enum CompilationFlag {
    FromString { s: String },
    CSTD { std: CSTD },
    CPPSTD { std: CPPSTD },
    IncludeDir { include_dir: String },

    Flag { flag: Flag },
}

impl CompilationFlag {
    /// creates a flag from a string
    fn from_string(s: impl Into<String>) -> Self {
        Self::FromString { s: s.into() }
    }
}

pub enum LinkFlag {
    FromString { s: String },
    LibLocation { s: String },
    Lib { name: String },
}

impl LinkFlag {
    /// creates a flag from a string
    fn from_string(s: impl Into<String>) -> Self {
        Self::FromString { s: s.into() }
    }
}

pub struct CompilationFlags {
    flags: Vec<CompilationFlag>,
}

impl CompilationFlags {
    #[inline]
    pub fn empty() -> Self {
        Self::new(vec![])
    }
    #[inline]
    pub fn new(flags: Vec<CompilationFlag>) -> Self {
        Self { flags }
    }

    pub(crate) fn into_flags_iter(self) -> impl Iterator<Item = CompilationFlag> {
        self.flags.into_iter()
    }
}

pub struct LinkFlags {
    flags: Vec<LinkFlag>,
}

impl LinkFlags {
    #[inline]
    pub fn empty() -> Self {
        Self::new(vec![])
    }
    #[inline]
    pub fn new(flags: Vec<LinkFlag>) -> Self {
        Self { flags }
    }

    pub(crate) fn into_flags_iter(self) -> impl Iterator<Item = LinkFlag> {
        self.flags.into_iter()
    }
}
