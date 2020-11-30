pub trait SomeNoneIf<Target> {
    fn some_if(self, condition: bool) -> Option<Target>;
    fn none_if(self, condition: bool) -> Option<Target>;
}

pub trait SomeNoneIfOwned: ToOwned {
    fn some_if_owned(&self, condition: bool) -> Option<Self::Owned>;
    fn none_if_owned(&self, condition: bool) -> Option<Self::Owned>;
}

impl<T> SomeNoneIf<T> for T {
    fn some_if(self, condition: bool) -> Option<Self> {
        if condition {
            Some(self)
        } else {
            None
        }
    }

    fn none_if(self, condition: bool) -> Option<Self> {
        if condition {
            None
        } else {
            Some(self)
        }
    }
}

impl<T> SomeNoneIfOwned for T
where
    T: ToOwned,
{
    fn some_if_owned(&self, condition: bool) -> Option<Self::Owned> {
        if condition {
            Some(self.to_owned())
        } else {
            None
        }
    }

    fn none_if_owned(&self, condition: bool) -> Option<Self::Owned> {
        if condition {
            None
        } else {
            Some(self.to_owned())
        }
    }
}

pub trait AndThenDo {
    type VT;
    fn and_then_do<F, U>(self, f: F)
    where
        F: FnOnce(Self::VT) -> U;
}

impl<T, E> AndThenDo for Result<T, E> {
    type VT = T;

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

    fn and_then_do<F, U>(self, f: F)
    where
        F: FnOnce(T) -> U,
    {
        if let Some(v) = self {
            let _ = f(v);
        }
    }
}
