//! FIXME: docs
/// A parse error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError(pub Box<String>);

impl<T> From<T> for ParseError
where
    T: Into<String>,
{
    fn from(s: T) -> Self {
        Self(Box::new(s.into()))
    }
}
