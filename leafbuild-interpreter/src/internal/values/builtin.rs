#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum BuiltinTy {
    Module,
    Project,
}

impl fmt::Display for BuiltinTy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.get_name())
    }
}

impl BuiltinTy {
    pub const fn get_name(self) -> Cow<'static, str> {
        match self {
            Self::Module => Cow::Borrowed("module"),
            Self::Project => Cow::Borrowed("project"),
        }
    }
    pub fn get_blueprint(self) -> &'static TyBlueprint {
        match self {
            Self::Module => &MODULE_TY_BLUEPRINT,
            Self::Project => &PROJECT_TY_BLUEPRINT,
        }
    }
}

include! {"project.rs"}
include! {"module.rs"}
