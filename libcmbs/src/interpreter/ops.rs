use crate::interpreter::{Value, ValueTypeMarker};

pub(crate) trait OpAdd<T, Result>
    where T: ValueTypeMarker, Result: ValueTypeMarker {
    fn add_to(&self, val: Value<T>) -> Value<Result>;
}

pub(crate) trait OpSubtract<T, Result>
    where T: ValueTypeMarker, Result: ValueTypeMarker {
    fn subtract(&self, val: Value<T>) -> Value<Result>;
}

pub(crate) trait OpMultiply<T, Result>
    where T: ValueTypeMarker, Result: ValueTypeMarker {
    fn multiply_with(&self, val: Value<T>) -> Value<Result>;
}

pub(crate) trait OpDivide<T, Result>
    where T: ValueTypeMarker, Result: ValueTypeMarker {
    fn divide_by(&self, val: Value<T>) -> Value<Result>;
}

pub(crate) trait OpModulo<T, Result>
    where T: ValueTypeMarker, Result: ValueTypeMarker {
    fn modulo(&self, val: Value<T>) -> Value<Result>;
}