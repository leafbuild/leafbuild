pub(crate) struct ParseError(pub Box<String>);

impl<T> From<T> for ParseError
where
    T: Into<String>,
{
    fn from(s: T) -> Self {
        Self(Box::new(s.into()))
    }
}
