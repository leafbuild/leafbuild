//! Kotlin-like `also` expressions.

/// Trait to implement `also` on all types
pub trait Also: Sized {
    /// Kotlin-like `also` extension function
    #[inline(always)]
    fn also<F, T>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self) -> T,
    {
        let _ = f(&mut self);
        self
    }
}

impl<T> Also for T {}
