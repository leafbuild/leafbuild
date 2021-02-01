/// Defines `take_if` and `take_unless`, which return
/// an option with the value depending on the
/// condition
pub trait TakeIfUnless: Sized {
    /// Returns `Some(self)` if `condition(&self)` or None otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use leafbuild_core::prelude::*;
    ///
    /// assert_eq!(0_i32.take_if(|it| it % 2 == 1), None);
    /// assert_eq!(1_i32.take_if(|it| it % 2 == 1), Some(1));
    /// assert_eq!("".take_if(|it| it.is_empty()), Some(""));
    /// assert_eq!("1,2,3".take_if(|it| it.is_empty()), None);
    /// ```
    #[inline(always)]
    fn take_if(self, condition: impl FnOnce(&Self) -> bool) -> Option<Self> {
        if condition(&self) {
            Some(self)
        } else {
            None
        }
    }

    /// Returns `None` if `condition(&self)` or `Some(self)` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use leafbuild_core::prelude::*;
    ///
    /// assert_eq!(0_i32.take_unless(|it| it % 2 == 1), Some(0));
    /// assert_eq!(1_i32.take_unless(|it| it % 2 == 1), None);
    /// assert_eq!("".take_unless(|it| it.is_empty()), None);
    /// assert_eq!("1,2,3".take_unless(|it| it.is_empty()), Some("1,2,3"));
    /// ```
    #[inline(always)]
    fn take_unless(self, condition: impl FnOnce(&Self) -> bool) -> Option<Self> {
        if condition(&self) {
            None
        } else {
            Some(self)
        }
    }
}

impl<T: Sized> TakeIfUnless for T {}
