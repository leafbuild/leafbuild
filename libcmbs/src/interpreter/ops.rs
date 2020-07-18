use crate::interpreter::{types::TypeId, Value, ValueTypeMarker};
use std::convert::TryFrom;
use std::ops::Deref;

pub(crate) fn op_add(
    ls: Value<Box<dyn ValueTypeMarker>>,
    rs: Value<Box<dyn ValueTypeMarker>>,
) -> Value<Box<dyn ValueTypeMarker>> {
    match (ls.get_value().get_type_id(), rs.get_value().get_type_id()) {
        (TypeId::String(left), right) => {
            Value::new(Box::new(format!("{}{}", left, rs.stringify())))
        }
        (left, TypeId::String(right)) => {
            Value::new(Box::new(format!("{}{}", ls.stringify(), right)))
        }
        (TypeId::I32(leftval), right) => match right {
            TypeId::I32(rightval) => Value::new(Box::new(*leftval + *rightval)),
            TypeId::I64(rightval) => Value::new(Box::new(*leftval as i64 + *rightval)),
            _ => {
                panic!("Cannot add {} to {}", TypeId::I32(leftval), right);
            }
        },
        (TypeId::I64(leftval), right) => match right {
            TypeId::I32(rightval) => Value::new(Box::new(*leftval + (*rightval as i64))),
            TypeId::I64(rightval) => Value::new(Box::new(*leftval + *rightval)),
            _ => {
                panic!("Cannot add {} to {}", TypeId::I64(leftval), right);
            }
        },
        (TypeId::U32(leftval), right) => match right {
            TypeId::U32(rightval) => Value::new(Box::new(*leftval + *rightval)),
            TypeId::U64(rightval) => Value::new(Box::new(*leftval as u64 + *rightval)),
            _ => {
                panic!("Cannot add {} to {}", TypeId::U32(leftval), right);
            }
        },
        (TypeId::U64(leftval), right) => match right {
            TypeId::U32(rightval) => Value::new(Box::new(*leftval + (*rightval as u64))),
            TypeId::U64(rightval) => Value::new(Box::new(*leftval + *rightval)),
            _ => {
                panic!("Cannot add {} to {}", TypeId::U64(leftval), right);
            }
        },
    }
}

pub(crate) fn op_sub(
    ls: Value<Box<dyn ValueTypeMarker>>,
    rs: Value<Box<dyn ValueTypeMarker>>,
) -> Value<Box<dyn ValueTypeMarker>> {
    match (ls.get_value().get_type_id(), rs.get_value().get_type_id()) {
        (TypeId::I32(leftval), right) => match right {
            TypeId::I32(rightval) => Value::new(Box::new(*leftval - *rightval)),
            TypeId::I64(rightval) => Value::new(Box::new(*leftval as i64 - *rightval)),
            _ => {
                panic!("Cannot perform '{} - {}'", TypeId::I32(leftval), right);
            }
        },
        (TypeId::I64(leftval), right) => match right {
            TypeId::I32(rightval) => Value::new(Box::new(*leftval - (*rightval as i64))),
            TypeId::I64(rightval) => Value::new(Box::new(*leftval - *rightval)),
            _ => {
                panic!("Cannot perform '{} - {}'", TypeId::I64(leftval), right);
            }
        },
        (TypeId::U32(leftval), right) => match right {
            TypeId::U32(rightval) => Value::new(Box::new(*leftval - *rightval)),
            TypeId::U64(rightval) => Value::new(Box::new(*leftval as u64 - *rightval)),
            _ => {
                panic!("Cannot perform '{} - {}'", TypeId::U32(leftval), right);
            }
        },
        (TypeId::U64(leftval), right) => match right {
            TypeId::U32(rightval) => Value::new(Box::new(*leftval - (*rightval as u64))),
            TypeId::U64(rightval) => Value::new(Box::new(*leftval - *rightval)),
            _ => {
                panic!("Cannot perform '{} - {}'", TypeId::U64(leftval), right);
            }
        },
        (left, right) => panic!("Cannot perform '{} - {}'", right, left),
    }
}

pub(crate) fn op_mul(
    ls: Value<Box<dyn ValueTypeMarker>>,
    rs: Value<Box<dyn ValueTypeMarker>>,
) -> Value<Box<dyn ValueTypeMarker>> {
    match (ls.get_value().get_type_id(), rs.get_value().get_type_id()) {
        (TypeId::I32(leftval), right) => match right {
            TypeId::I32(rightval) => Value::new(Box::new(*leftval * *rightval)),
            TypeId::I64(rightval) => Value::new(Box::new(*leftval as i64 * *rightval)),
            _ => {
                panic!("Cannot perform '{} * {}'", TypeId::I32(leftval), right);
            }
        },
        (TypeId::I64(leftval), right) => match right {
            TypeId::I32(rightval) => Value::new(Box::new(*leftval * (*rightval as i64))),
            TypeId::I64(rightval) => Value::new(Box::new(*leftval * *rightval)),
            _ => {
                panic!("Cannot perform '{} * {}'", TypeId::I64(leftval), right);
            }
        },
        (TypeId::U32(leftval), right) => match right {
            TypeId::U32(rightval) => Value::new(Box::new(*leftval * *rightval)),
            TypeId::U64(rightval) => Value::new(Box::new(*leftval as u64 * *rightval)),
            _ => {
                panic!("Cannot perform '{} * {}'", TypeId::U32(leftval), right);
            }
        },
        (TypeId::U64(leftval), right) => match right {
            TypeId::U32(rightval) => Value::new(Box::new(*leftval * (*rightval as u64))),
            TypeId::U64(rightval) => Value::new(Box::new(*leftval * *rightval)),
            _ => {
                panic!("Cannot perform '{} * {}'", TypeId::U64(leftval), right);
            }
        },
        (left, right) => panic!("Cannot perform '{} * {}'", right, left),
    }
}

pub(crate) fn op_div(
    ls: Value<Box<dyn ValueTypeMarker>>,
    rs: Value<Box<dyn ValueTypeMarker>>,
) -> Value<Box<dyn ValueTypeMarker>> {
    match (ls.get_value().get_type_id(), rs.get_value().get_type_id()) {
        (TypeId::I32(leftval), right) => match right {
            TypeId::I32(rightval) => Value::new(Box::new(*leftval / *rightval)),
            TypeId::I64(rightval) => Value::new(Box::new(*leftval as i64 / *rightval)),
            _ => {
                panic!("Cannot perform '{} / {}'", TypeId::I32(leftval), right);
            }
        },
        (TypeId::I64(leftval), right) => match right {
            TypeId::I32(rightval) => Value::new(Box::new(*leftval / (*rightval as i64))),
            TypeId::I64(rightval) => Value::new(Box::new(*leftval / *rightval)),
            _ => {
                panic!("Cannot perform '{} / {}'", TypeId::I64(leftval), right);
            }
        },
        (TypeId::U32(leftval), right) => match right {
            TypeId::U32(rightval) => Value::new(Box::new(*leftval / *rightval)),
            TypeId::U64(rightval) => Value::new(Box::new(*leftval as u64 / *rightval)),
            _ => {
                panic!("Cannot perform '{} / {}'", TypeId::U32(leftval), right);
            }
        },
        (TypeId::U64(leftval), right) => match right {
            TypeId::U32(rightval) => Value::new(Box::new(*leftval / (*rightval as u64))),
            TypeId::U64(rightval) => Value::new(Box::new(*leftval / *rightval)),
            _ => {
                panic!("Cannot perform '{} / {}'", TypeId::U64(leftval), right);
            }
        },
        (left, right) => panic!("Cannot perform '{} / {}'", right, left),
    }
}

pub(crate) fn op_mod(
    ls: Value<Box<dyn ValueTypeMarker>>,
    rs: Value<Box<dyn ValueTypeMarker>>,
) -> Value<Box<dyn ValueTypeMarker>> {
    match (ls.get_value().get_type_id(), rs.get_value().get_type_id()) {
        (TypeId::I32(leftval), right) => match right {
            TypeId::I32(rightval) => Value::new(Box::new(*leftval % *rightval)),
            TypeId::I64(rightval) => Value::new(Box::new(*leftval as i64 % *rightval)),
            _ => {
                panic!("Cannot perform '{} % {}'", TypeId::I32(leftval), right);
            }
        },
        (TypeId::I64(leftval), right) => match right {
            TypeId::I32(rightval) => Value::new(Box::new(*leftval % (*rightval as i64))),
            TypeId::I64(rightval) => Value::new(Box::new(*leftval % *rightval)),
            _ => {
                panic!("Cannot perform '{} % {}'", TypeId::I64(leftval), right);
            }
        },
        (TypeId::U32(leftval), right) => match right {
            TypeId::U32(rightval) => Value::new(Box::new(*leftval % *rightval)),
            TypeId::U64(rightval) => Value::new(Box::new(*leftval as u64 % *rightval)),
            _ => {
                panic!("Cannot perform '{} % {}'", TypeId::U32(leftval), right);
            }
        },
        (TypeId::U64(leftval), right) => match right {
            TypeId::U32(rightval) => Value::new(Box::new(*leftval % (*rightval as u64))),
            TypeId::U64(rightval) => Value::new(Box::new(*leftval % *rightval)),
            _ => {
                panic!("Cannot perform '{} % {}'", TypeId::U64(leftval), right);
            }
        },
        (left, right) => panic!("Cannot perform '{} % {}'", right, left),
    }
}
