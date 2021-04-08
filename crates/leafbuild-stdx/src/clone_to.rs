//! Holds the [`CloneTo`] trait

/// Trait to define `clone_to*` on all types
pub trait CloneTo: Clone {
    /// Clones other from self and returns self; reference variant.
    ///
    /// # Examples
    /// ```
    /// # use leafbuild_stdx::CloneTo;
    /// let mut  a = 0;
    /// assert_eq!(&1, 1_i32.clone_to_ref(&mut a));
    /// assert_eq!(1, a);
    /// ```
    #[inline(always)]
    fn clone_to_ref(&self, other: &mut Self) -> &Self {
        other.clone_from(self);
        self
    }

    /// Clones other from self and returns self; mutable reference variant.
    ///
    /// # Examples
    /// ```
    /// # use leafbuild_stdx::CloneTo;
    /// let mut a = 0;
    /// assert_eq!(&mut 1, 1_i32.clone_to_ref_mut(&mut a));
    /// assert_eq!(1, a);
    /// ```
    #[inline]
    fn clone_to_ref_mut(&mut self, other: &mut Self) -> &mut Self {
        other.clone_from(self);
        self
    }

    /// Clones other from self and returns self; owned variant.
    ///
    /// # Examples
    /// ```
    /// # use leafbuild_stdx::CloneTo;
    /// let mut a = 0;
    /// assert_eq!(1, 1_i32.clone_to(&mut a));
    /// assert_eq!(1, a);
    /// ```
    #[inline(always)]
    fn clone_to(self, other: &mut Self) -> Self {
        other.clone_from(&self);
        self
    }
}

impl<T: Clone> CloneTo for T {}
