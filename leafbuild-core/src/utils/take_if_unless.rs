/// Defines `take_if` and `take_unless`, which return
/// an option with the value depending on the
/// condition
pub trait TakeIfUnless: Sized {
    /// Returns `Some(...)` if condition == true or None otherwise
    #[inline(always)]
    fn take_if<F>(self, condition: F) -> Option<Self>
    where
        F: FnOnce(&Self) -> bool,
    {
        if condition(&self) {
            Some(self)
        } else {
            None
        }
    }
    /// Returns `None` if condition == true or Some(...) otherwise
    #[inline(always)]
    fn take_unless<F>(self, condition: F) -> Option<Self>
    where
        F: FnOnce(&Self) -> bool,
    {
        if condition(&self) {
            None
        } else {
            Some(self)
        }
    }
}

impl<T> TakeIfUnless for T {}
