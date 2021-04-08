//! Kotlin-like `let` expressions.

/// Trait to implement `let` on all types.
pub trait Let: Sized {
    /// Convert this object with a closure
    ///
    /// # Examples
    ///
    /// ```
    /// # use leafbuild_stdx::Let;
    /// assert_eq!(2, 1.let_(|it| it + 1));
    /// ```
    #[inline(always)]
    fn let_<F, T>(self, f: F) -> T
    where
        F: FnOnce(Self) -> T,
    {
        f(self)
    }
}

/// Trait to implement `let_ref` on all types.
pub trait LetRef {
    /// Convert a reference with a closure
    ///
    /// # Examples
    ///
    /// ```
    /// # use leafbuild_stdx::LetRef;
    /// assert_eq!(String::from("aaa"), "aaa".let_ref(|it| it.to_string()));
    /// ```
    #[inline(always)]
    fn let_ref<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&Self) -> T,
    {
        f(self)
    }
}

/// Trait to implement `let_ref_mut` on all types.
pub trait LetRefMut {
    /// Convert a mutable reference with a closure
    ///
    /// # Examples
    ///
    /// ```
    /// # use leafbuild_stdx::LetRefMut;
    /// # #[derive(Debug)]
    /// #[derive(Eq, PartialEq)]
    /// struct P(i32);
    /// let mut  p = P(0);
    /// assert_eq!(1, p.let_ref_mut(|it| {it.0 = 1; it.0}));
    /// assert_eq!(P(1), p);
    /// ```
    #[inline(always)]
    fn let_ref_mut<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        f(self)
    }
}

impl<T> Let for T {}
impl<T> LetRef for T {}
impl<T> LetRefMut for T {}
