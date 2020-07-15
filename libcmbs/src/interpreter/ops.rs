use crate::interpreter::{ValType, Value};

pub trait OpPlus<T, Result>
    where T: ValType, Result: ValType {
    fn plus(val: Value<T>) -> Value<Result>;
}