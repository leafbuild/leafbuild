pub trait CompilationFlag {
    /// creates a flag from a string
    fn from_string(s: impl Into<String>) -> Self;
}

pub trait LinkFlag {
    /// creates a flag from a string
    fn from_string(s: impl Into<String>) -> Self;
}

pub struct CompilationFlags<T>
where
    T: CompilationFlag,
{
    flags: Vec<T>,
}

impl<T> CompilationFlags<T>
where
    T: CompilationFlag,
{
    #[inline]
    pub fn empty() -> Self {
        Self::new(vec![])
    }
    #[inline]
    pub fn new(flags: Vec<T>) -> Self {
        Self { flags }
    }
}

pub struct LinkFlags<T>
where
    T: LinkFlag,
{
    flags: Vec<T>,
}

impl<T> LinkFlags<T>
where
    T: LinkFlag,
{
    #[inline]
    pub fn empty() -> Self {
        Self::new(vec![])
    }
    #[inline]
    pub fn new(flags: Vec<T>) -> Self {
        Self { flags }
    }
}
