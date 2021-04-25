/// Copies the value and returns it, so no need to go back and use `*`.
pub trait Copied: Sized + Copy {
    #[inline(always)]
    fn copied_val(&self) -> Self {
        *self
    }
}
