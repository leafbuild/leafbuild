/*
 *   Copyright (c) 2021 Dinu Blanovschi
 *   All rights reserved.
 *   Licensed under the terms of the BSD-3 Clause license, see LICENSE for more.
 */
/// A trait for exposing [`AndThenDo::and_then_do`] on [`Option`] and [`Result`]
pub trait AndThenDo {
    /// The type of the value
    type VT;
    /// Do an action on the `Ok`/`Some` value and don't ignore
    /// the returned value
    fn and_then_do<F, U>(self, f: F)
    where
        F: FnOnce(Self::VT) -> U;
}

impl<T, E> AndThenDo for Result<T, E> {
    type VT = T;

    #[inline(always)]
    fn and_then_do<F, U>(self, f: F)
    where
        F: FnOnce(T) -> U,
    {
        if let Ok(v) = self {
            let _ = f(v);
        }
    }
}

impl<T> AndThenDo for Option<T> {
    type VT = T;

    #[inline(always)]
    fn and_then_do<F, U>(self, f: F)
    where
        F: FnOnce(T) -> U,
    {
        if let Some(v) = self {
            let _ = f(v);
        }
    }
}
