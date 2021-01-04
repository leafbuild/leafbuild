//! Kotlin-like `let` expressions.

/// Trait to implement `let` on all types.
pub trait Let: Sized {
    /// Convert this object with a closure
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
