/// Defines `some_if` and `none_if`, which return
/// an option with the value depending on the
/// condition
pub trait SomeNoneIf {
    /// Returns `Some(...)` if condition == true or None otherwise
    fn some_if(self, condition: bool) -> Option<Self>
    where
        Self: Sized;
    /// Returns `None` if condition == true or Some(...) otherwise
    fn none_if(self, condition: bool) -> Option<Self>
    where
        Self: Sized;
}

impl<T> SomeNoneIf for T {
    #[inline(always)]
    fn some_if(self, condition: bool) -> Option<Self> {
        if condition {
            Some(self)
        } else {
            None
        }
    }

    #[inline(always)]
    fn none_if(self, condition: bool) -> Option<Self> {
        if condition {
            None
        } else {
            Some(self)
        }
    }
}
