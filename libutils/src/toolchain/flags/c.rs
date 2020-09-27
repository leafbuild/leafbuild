pub enum CSTD {
    // ANSI = C89 = C90
    ANSI,

    C99,
    GNU99,

    C11,
    GNU11,
    // TODO: add more here as they come
}

impl CSTD {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            CSTD::ANSI => "ansi",
            CSTD::C99 => "c99",
            CSTD::GNU99 => "gnu99",
            CSTD::C11 => "c11",
            CSTD::GNU11 => "gnu11",
        }
    }
}

pub enum CFlag {
    PositionIndependentCode,
}

pub enum CCompilationFlag {
    FromString { s: String },
    CSTD { std: CSTD },
    IncludeDir { include_dir: String },

    Flag { flag: CFlag },

    None,
}

impl CCompilationFlag {
    /// creates a flag from a string
    fn from_string(s: impl Into<String>) -> Self {
        Self::FromString { s: s.into() }
    }
}

pub enum CLinkFlag {
    FromString { s: String },
    LibLocation { s: String },
    Lib { name: String },
    LibShared,
    None,
}

impl CLinkFlag {
    /// creates a flag from a string
    fn from_string(s: impl Into<String>) -> Self {
        Self::FromString { s: s.into() }
    }
}

pub struct CCompilationFlags {
    flags: Vec<CCompilationFlag>,
}

impl CCompilationFlags {
    #[inline]
    pub fn empty() -> Self {
        Self::new(vec![])
    }
    #[inline]
    pub fn new(flags: Vec<CCompilationFlag>) -> Self {
        Self { flags }
    }

    pub(crate) fn into_flags_iter(self) -> impl Iterator<Item = CCompilationFlag> {
        self.flags.into_iter()
    }
}

pub struct CLinkFlags {
    flags: Vec<CLinkFlag>,
}

impl CLinkFlags {
    #[inline]
    pub fn empty() -> Self {
        Self::new(vec![])
    }
    #[inline]
    pub fn new(flags: Vec<CLinkFlag>) -> Self {
        Self { flags }
    }

    pub(crate) fn into_flags_iter(self) -> impl Iterator<Item = CLinkFlag> {
        self.flags.into_iter()
    }
}
