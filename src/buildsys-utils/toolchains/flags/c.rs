pub enum STD {
    // ANSI = C89 = C90
    ANSI,

    C99,
    GNU99,

    C11,
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

pub enum Flag {
    PositionIndependentCode,
}

pub enum CompilationFlag {
    FromString { s: String },
    CSTD { std: STD },
    IncludeDir { include_dir: String },

    Flag { flag: Flag },

    None,
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
    LibShared,
    None,
}

impl LinkFlag {
    /// creates a flag from a string
    pub fn from_string(s: impl Into<String>) -> Self {
        Self::FromString { s: s.into() }
    }
}

pub struct CompilationFlags {
    flags: Vec<CompilationFlag>,
}

impl CompilationFlags {
    #[must_use]
    pub const fn empty() -> Self {
        Self::new(vec![])
    }
    #[must_use]
    pub const fn new(flags: Vec<CompilationFlag>) -> Self {
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
    #[must_use]
    pub const fn empty() -> Self {
        Self::new(vec![])
    }
    #[must_use]
    pub const fn new(flags: Vec<LinkFlag>) -> Self {
        Self { flags }
    }

    pub(crate) fn into_flags_iter(self) -> impl Iterator<Item = LinkFlag> {
        self.flags.into_iter()
    }
}
