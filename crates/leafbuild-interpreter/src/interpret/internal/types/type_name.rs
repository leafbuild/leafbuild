pub trait TypeNameTrait {
    fn name(&self) -> &str;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TypeName(pub String);

impl TypeName {
    fn as_ref(&self) -> TypeNameRef {
        TypeNameRef(&self.0)
    }
}

impl TypeNameTrait for TypeName {
    fn name(&self) -> &str {
        &self.0
    }
}

impl<'a> From<&'a str> for TypeName {
    fn from(name: &'a str) -> Self {
        Self(name.to_string())
    }
}

impl From<String> for TypeName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TypeNameRef<'a>(pub &'a str);

impl<'a> TypeNameRef<'a> {
    pub fn to_owned(self) -> TypeName {
        TypeName(self.0.to_string())
    }
}

impl<'a> TypeNameTrait for TypeNameRef<'a> {
    fn name(&self) -> &str {
        self.0
    }
}

impl<'a> From<&'a str> for TypeNameRef<'a> {
    fn from(name: &'a str) -> Self {
        Self(name)
    }
}
