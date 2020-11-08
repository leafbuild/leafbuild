#[derive(Copy, Clone, Debug)]
pub enum Language {
    C,
    CPP,
}

impl Language {
    #[must_use]
    pub const fn get_compilation_flags_varname(self) -> &'static str {
        match self {
            Self::C => "CC_FLAGS",
            Self::CPP => "CXX_FLAGS",
        }
    }
    #[must_use]
    pub const fn get_link_flags_varname(self) -> &'static str {
        match self {
            Self::C => "CCLD_FLAGS",
            Self::CPP => "CXXLD_FLAGS",
        }
    }
}

pub struct NotALanguageError {
    msg: String,
}

impl NotALanguageError {
    pub(crate) fn new(s: impl Into<String>) -> Self {
        Self { msg: s.into() }
    }

    #[must_use]
    pub const fn get_msg(&self) -> &String {
        &self.msg
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
