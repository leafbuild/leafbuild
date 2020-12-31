/// A path to a leafbuild module.
#[derive(Default, Debug, Clone)]
pub struct LfModName(pub String);

impl LfModName {
    /// Creates a new [`LfModName`] from anything that can be turned into a string
    #[must_use]
    pub fn new(modname: impl Into<String>) -> Self {
        Self(modname.into())
    }
}
