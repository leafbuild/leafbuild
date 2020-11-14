/// A language
#[derive(Copy, Clone, Debug)]
pub enum Language {
    /// C
    C,
    /// C++
    CPP,
}

impl Language {
    /// The compilation flags varname.
    /// **Specific** to leafbuild
    #[must_use]
    pub const fn get_compilation_flags_varname(self) -> &'static str {
        match self {
            Self::C => "CC_FLAGS",
            Self::CPP => "CXX_FLAGS",
        }
    }

    /// The linking flags varname
    /// **Specific** to leafbuild
    #[must_use]
    pub const fn get_link_flags_varname(self) -> &'static str {
        match self {
            Self::C => "CCLD_FLAGS",
            Self::CPP => "CXXLD_FLAGS",
        }
    }
}

/// Returned by [`Language::from_str`] when the string isn't a recognized language
#[derive(Error, Debug)]
#[error("not a language `{lang}`")]
pub struct NotALanguageError {
    lang: String,
}

impl NotALanguageError {
    pub(crate) fn new(s: impl Into<String>) -> Self {
        Self { lang: s.into() }
    }
}

impl FromStr for Language {
    type Err = NotALanguageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" | "C" => Ok(Self::C),
            "c++" | "C++" | "cpp" | "CPP" => Ok(Self::CPP),
            other => Err(NotALanguageError::new(format!(
                "Cannot parse language from '{}'",
                other
            ))),
        }
    }
}
