//! Defines and implements traits for the [`Span`] data type
use derive_new::new;
use std::fmt;
use std::ops::Range;

/// A span in the source code
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, new)]
pub struct Span {
    start: usize,
    end: usize,
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl Span {
    /// Gets this [`Span`] as a [`Range`] of [`usize`]
    #[must_use]
    pub const fn get_rng(&self) -> Range<usize> {
        self.start..self.end
    }

    /// Gets the start of this [`Span`]
    #[must_use]
    pub const fn get_start(&self) -> usize {
        self.start
    }

    /// Gets the end of this [`Span`]
    #[must_use]
    pub const fn get_end(&self) -> usize {
        self.end
    }
}

impl From<Range<usize>> for Span {
    fn from(r: Range<usize>) -> Self {
        Self {
            start: r.start,
            end: r.end,
        }
    }
}

#[cfg(test)]
#[allow(clippy::fallible_impl_from)]
impl From<Range<i32>> for Span {
    fn from(r: Range<i32>) -> Self {
        use std::convert::TryInto;
        Self {
            start: r.start.try_into().unwrap(),
            end: r.end.try_into().unwrap(),
        }
    }
}
