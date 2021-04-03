/*
 *   Copyright (c) 2021 Dinu Blanovschi
 *   All rights reserved.
 *   Licensed under the terms of the BSD-3 Clause license, see LICENSE for more.
 */
//! Holds the [`CopyTo`] trait.

/// A simple trait that with `copy_to`.
pub trait CopyTo: Copy {
    /// Copies self to other and returns self
    ///
    /// # Examples
    /// ```
    /// # use leafbuild_stdx::CopyTo;
    /// #
    /// let mut  i = 1;
    /// assert_eq!(2, 2.copy_to(&mut i));
    /// assert_eq!(i, 2);
    /// ```
    #[inline(always)]
    fn copy_to(self, other: &mut Self) -> Self {
        *other = self;
        self
    }
}

impl<T: Copy> CopyTo for T {}
