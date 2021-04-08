/// Similar to [`SomeNoneIf`][`super::SomeNoneIf`], but works with the owned types
pub trait TakeIfUnlessOwned: ToOwned {
    /// Similar to [`SomeNoneIf::some_if`][`super::SomeNoneIf::some_if`], but works with owned types
    #[inline(always)]
    fn take_if_owned<F>(&self, condition: F) -> Option<Self::Owned>
    where
        F: FnOnce(&Self) -> bool,
    {
        if condition(self) {
            Some(self.to_owned())
        } else {
            None
        }
    }
    /// Similar to [`SomeNoneIf::none_if`][`super::SomeNoneIf::none_if`], but works with owned types
    #[inline(always)]
    fn take_unless_owned<F>(&self, condition: F) -> Option<Self::Owned>
    where
        F: FnOnce(&Self) -> bool,
    {
        if condition(self) {
            None
        } else {
            Some(self.to_owned())
        }
    }
}

impl<T: ToOwned + ?Sized> TakeIfUnlessOwned for T {}
