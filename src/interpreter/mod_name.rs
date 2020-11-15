/// A path to a leafbuild module.
#[derive(Default, Debug, Clone)]
pub struct LfModName(pub(crate) String);

impl LfModName {
    pub(crate) fn new(modname: impl Into<String>) -> Self {
        Self(modname.into())
    }
}
