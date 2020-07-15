use crate::interpreter::{Value, ValueTypeMarker};

pub(crate) trait OpPlus<T, Result>
    where T: ValueTypeMarker, Result: ValueTypeMarker {
    fn plus(val: Value<T>) -> Value<Result>;
}