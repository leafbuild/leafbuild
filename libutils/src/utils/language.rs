#[derive(Copy, Clone)]
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
