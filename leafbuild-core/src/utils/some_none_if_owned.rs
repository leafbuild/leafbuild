/// Similar to [`SomeNoneIf`], but works with the owned types
pub trait SomeNoneIfOwned: ToOwned {
    /// Similar to [`SomeNoneIf::some_if`], but works with owned types
    fn some_if_owned(&self, condition: bool) -> Option<Self::Owned>;
    /// Similar to [`SomeNoneIf::none_if`], but works with owned types
    fn none_if_owned(&self, condition: bool) -> Option<Self::Owned>;
}

impl<T> SomeNoneIfOwned for T
where
    T: ToOwned,
{
    #[inline(always)]
    fn some_if_owned(&self, condition: bool) -> Option<Self::Owned> {
        if condition {
            Some(self.to_owned())
        } else {
            None
        }
    }

    #[inline(always)]
    fn none_if_owned(&self, condition: bool) -> Option<Self::Owned> {
        if condition {
            None
        } else {
            Some(self.to_owned())
        }
    }
}
