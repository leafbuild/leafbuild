/// Similar to [`TakeIfUnless`][`super::TakeIfUnless`], but works with the owned types
pub trait TakeIfUnlessOwned: ToOwned {
    /// Similar to [`TakeIfUnless::take_if`][`super::TakeIfUnless::take_if`], but works with owned types
    ///
    /// # Examples
    ///
    /// ```
    /// use leafbuild_core::prelude::*;
    /// assert_eq!("1,2,3".take_if_owned(|it| it.is_empty()), None);
    /// assert_eq!("".take_if_owned(|it| it.is_empty()), Some(String::from("")));
    /// ```
    #[inline(always)]
    fn take_if_owned(&self, condition: impl FnOnce(&Self) -> bool) -> Option<Self::Owned> {
        use super::TakeIfUnless;
        self.take_if(|it| condition(*it)).map(ToOwned::to_owned)
    }
    /// Similar to [`TakeIfUnless::take_unless`][`super::TakeIfUnless::take_unless`], but works with owned types
    ///
    /// # Examples
    ///
    /// ```
    /// use leafbuild_core::prelude::*;
    /// assert_eq!("1,2,3".take_unless_owned(|it| it.is_empty()), Some(String::from("1,2,3")));
    /// assert_eq!(     "".take_unless_owned(|it| it.is_empty()), None);
    /// ```
    #[inline(always)]
    fn take_unless_owned(&self, condition: impl FnOnce(&Self) -> bool) -> Option<Self::Owned> {
        use super::TakeIfUnless;
        self.take_unless(|it| condition(*it)).map(ToOwned::to_owned)
    }
}

impl<T: ToOwned + ?Sized> TakeIfUnlessOwned for T {}
