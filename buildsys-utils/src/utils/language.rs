#[derive(Copy, Clone, Debug)]
pub enum Language {
    C,
    CPP,
}

impl Language {
    pub fn get_compilation_flags_varname(&self) -> &'static str {
        match self {
            Language::C => "CC_FLAGS",
            Language::CPP => "CXX_FLAGS",
        }
    }
    pub fn get_link_flags_varname(&self) -> &'static str {
        match self {
            Language::C => "CCLD_FLAGS",
            Language::CPP => "CXXLD_FLAGS",
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

    pub fn get_msg(&self) -> &String {
        &self.msg
    }
}

impl FromStr for Language {
    type Err = NotALanguageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" | "C" => Ok(Language::C),
            "c++" | "C++" | "cpp" | "CPP" => Ok(Language::CPP),
            other => Err(NotALanguageError::new(format!(
                "Cannot parse language from '{}'",
                other
            ))),
        }
    }
}
