use crate::interpreter::types::TypeId;
use crate::interpreter::{types::TypeIdAndValue, EnvFrame, LaterValue, Value, ValueTypeMarker};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use std::ops::Range;

#[derive(Copy, Clone)]
pub(crate) enum OpsErrorType {
    Incompatible,
    IncompatibleError,
}

pub(crate) struct OpsError {
    type_: OpsErrorType,
    diagnostic: Diagnostic<usize>,
}

impl OpsError {
    fn new(type_: OpsErrorType, diagnostic: Diagnostic<usize>) -> OpsError {
        Self { type_, diagnostic }
    }
    pub(crate) fn get_type(&self) -> &OpsErrorType {
        &self.type_
    }
    pub(crate) fn get_diagnostic(self) -> Diagnostic<usize> {
        self.diagnostic
    }
}

macro_rules! op_match {
    ($a:expr, $a_rng:expr, $op:tt, $b:expr, $b_rng:expr, $file_id:expr $(, $($x:pat => $y:expr),*)?) => {
        match (
            $a.get_value().get_type_id_and_value(),
            $b.get_value().get_type_id_and_value(),
        ) {
            (TypeIdAndValue::Error, tright) => Err(OpsError::new(
                    OpsErrorType::IncompatibleError,
                    Diagnostic::error()
                        .with_message("Incompatible operands(one is an error)")
                        .with_labels(vec![
                            Label::primary($file_id, $a_rng)
                                .with_message(TypeId::Error.typename()),
                            Label::secondary($file_id, $b_rng)
                                .with_message(tright.degrade().typename()),
                        ]),
            )),
            (tleft, TypeIdAndValue::Error) => Err(OpsError::new(
                    OpsErrorType::IncompatibleError,
                    Diagnostic::error()
                        .with_message("Incompatible operands(one is an error)")
                        .with_labels(vec![
                            Label::primary($file_id, $a_rng)
                                .with_message(tleft.degrade().typename()),
                            Label::secondary($file_id, $b_rng)
                                .with_message(TypeId::Error.typename()),
                        ]),
            )),
            $($($x => $y),*,)?
            (TypeIdAndValue::I32(leftval), tright) => match tright {
                TypeIdAndValue::I32(rightval) => Ok(Value::new(Box::new(*leftval $op *rightval))),
                TypeIdAndValue::I64(rightval) => {
                    Ok(Value::new(Box::new(*leftval as i64 $op *rightval)))
                }
                _ => Err(OpsError::new(
                    OpsErrorType::Incompatible,
                    Diagnostic::error()
                        .with_message("Incompatible operands")
                        .with_labels(vec![
                            Label::primary($file_id, $a_rng)
                                .with_message(TypeIdAndValue::I32(leftval).degrade().typename()),
                            Label::secondary($file_id, $b_rng)
                                .with_message(tright.degrade().typename()),
                        ]),
                )),
            },
            (TypeIdAndValue::I64(leftval), tright) => match tright {
                TypeIdAndValue::I32(rightval) => {
                    Ok(Value::new(Box::new(*leftval $op (*rightval as i64))))
                }
                TypeIdAndValue::I64(rightval) => Ok(Value::new(Box::new(*leftval $op *rightval))),
                _ => Err(OpsError::new(
                    OpsErrorType::Incompatible,
                    Diagnostic::error()
                        .with_message("Incompatible operands")
                        .with_labels(vec![
                            Label::primary($file_id, $a_rng)
                                .with_message(TypeIdAndValue::I64(leftval).degrade().typename()),
                            Label::secondary($file_id, $b_rng)
                                .with_message(tright.degrade().typename()),
                        ]),
                )),
            },
            (TypeIdAndValue::U32(leftval), tright) => match tright {
                TypeIdAndValue::U32(rightval) => Ok(Value::new(Box::new(*leftval $op *rightval))),
                TypeIdAndValue::U64(rightval) => {
                    Ok(Value::new(Box::new(*leftval as u64 $op *rightval)))
                }
                _ => Err(OpsError::new(
                    OpsErrorType::Incompatible,
                    Diagnostic::error()
                        .with_message("Incompatible operands")
                        .with_labels(vec![
                            Label::primary($file_id, $a_rng)
                                .with_message(TypeIdAndValue::U32(leftval).degrade().typename()),
                            Label::secondary($file_id, $b_rng)
                                .with_message(tright.degrade().typename()),
                        ]),
                )),
            },
            (TypeIdAndValue::U64(leftval), tright) => match tright {
                TypeIdAndValue::U32(rightval) => {
                    Ok(Value::new(Box::new(*leftval $op (*rightval as u64))))
                }
                TypeIdAndValue::U64(rightval) => Ok(Value::new(Box::new(*leftval $op *rightval))),
                _ => Err(OpsError::new(
                    OpsErrorType::Incompatible,
                    Diagnostic::error()
                        .with_message("Incompatible operands")
                        .with_labels(vec![
                            Label::primary($file_id, $a_rng)
                                .with_message(TypeIdAndValue::U64(leftval).degrade().typename()),
                            Label::secondary($file_id, $b_rng)
                                .with_message(tright.degrade().typename()),
                        ]),
                )),
            },
            (tleft, tright) => Err(OpsError::new(
                OpsErrorType::Incompatible,
                Diagnostic::error()
                    .with_message("Incompatible operands")
                    .with_labels(vec![
                        Label::primary($file_id, $a_rng)
                            .with_message(tleft.degrade().typename()),
                        Label::secondary($file_id, $b_rng)
                            .with_message(tright.degrade().typename()),
                    ]),
            )),
        }
    };
}

pub(crate) fn op_add(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Range<usize>,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Range<usize>,
    file_id: usize,
) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
    op_match!(ls, left_range, +, rs, right_range, file_id,
    (TypeIdAndValue::String(left), tright) => Ok(Value::new(Box::new(format!(
        "{}{}",
        left,
        tright.stringify()
    )))),
    (tleft, TypeIdAndValue::String(right)) => Ok(Value::new(Box::new(format!(
        "{}{}",
        tleft.stringify(),
        right
    )))))
}

pub(crate) fn op_sub(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Range<usize>,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Range<usize>,
    file_id: usize,
) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
    op_match!(ls, left_range, -, rs, right_range, file_id)
}
pub(crate) fn op_mul(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Range<usize>,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Range<usize>,
    file_id: usize,
) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
    op_match!(ls, left_range, *, rs, right_range, file_id)
}
pub(crate) fn op_div(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Range<usize>,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Range<usize>,
    file_id: usize,
) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
    op_match!(ls, left_range, /, rs, right_range, file_id)
}
pub(crate) fn op_mod(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Range<usize>,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Range<usize>,
    file_id: usize,
) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
    op_match!(ls, left_range, %, rs, right_range, file_id)
}

pub(crate) fn op_and<'a>(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Range<usize>,
    rs: &LaterValue<'a>,
    right_range: Range<usize>,
    frame: &mut EnvFrame,
    file_id: usize,
) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
    match ls.get_value().get_type_id_and_value() {
        TypeIdAndValue::Error => Err(OpsError::new(
            OpsErrorType::IncompatibleError,
            Diagnostic::error()
                .with_message("Incompatible operands(left is an error)")
                .with_labels(vec![
                    Label::primary(file_id, left_range).with_message(TypeId::Error.typename())
                ]),
        )),
        TypeIdAndValue::Bool(left) => {
            if !left {
                return Ok(Value::new(Box::new(false)));
            }
            match rs.compute(frame).get_value().get_type_id_and_value() {
                TypeIdAndValue::Error => Err(OpsError::new(
                    OpsErrorType::IncompatibleError,
                    Diagnostic::error()
                        .with_message("Incompatible operands(right is an error)")
                        .with_labels(vec![Label::primary(file_id, right_range)
                            .with_message(TypeId::Error.typename())]),
                )),
                TypeIdAndValue::Bool(right) => Ok(Value::new(Box::new(*right))),
                tleft => Err(OpsError::new(
                    OpsErrorType::Incompatible,
                    Diagnostic::error()
                        .with_message("Non-bool right operand to `and`")
                        .with_labels(vec![Label::primary(file_id, right_range)
                            .with_message(tleft.degrade().typename())]),
                )),
            }
        }
        tleft => Err(OpsError::new(
            OpsErrorType::Incompatible,
            Diagnostic::error()
                .with_message("Non-bool left operand to `and`")
                .with_labels(vec![
                    Label::primary(file_id, left_range).with_message(tleft.degrade().typename())
                ]),
        )),
    }
}

pub(crate) fn op_or<'a>(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Range<usize>,
    rs: &LaterValue<'a>,
    right_range: Range<usize>,
    frame: &mut EnvFrame,
    file_id: usize,
) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
    match ls.get_value().get_type_id_and_value() {
        TypeIdAndValue::Error => Err(OpsError::new(
            OpsErrorType::IncompatibleError,
            Diagnostic::error()
                .with_message("Incompatible operands(left is an error)")
                .with_labels(vec![
                    Label::primary(file_id, left_range).with_message(TypeId::Error.typename())
                ]),
        )),
        TypeIdAndValue::Bool(left) => {
            if *left {
                return Ok(Value::new(Box::new(true)));
            }
            match rs.compute(frame).get_value().get_type_id_and_value() {
                TypeIdAndValue::Error => Err(OpsError::new(
                    OpsErrorType::IncompatibleError,
                    Diagnostic::error()
                        .with_message("Incompatible operands(right is an error)")
                        .with_labels(vec![Label::primary(file_id, right_range)
                            .with_message(TypeId::Error.typename())]),
                )),
                TypeIdAndValue::Bool(right) => Ok(Value::new(Box::new(*right))),
                tleft => Err(OpsError::new(
                    OpsErrorType::Incompatible,
                    Diagnostic::error()
                        .with_message("Non-bool right operand to `or`")
                        .with_labels(vec![Label::primary(file_id, right_range)
                            .with_message(tleft.degrade().typename())]),
                )),
            }
        }
        tleft => Err(OpsError::new(
            OpsErrorType::Incompatible,
            Diagnostic::error()
                .with_message("Non-left right operand to `or`")
                .with_labels(vec![
                    Label::primary(file_id, left_range).with_message(tleft.degrade().typename())
                ]),
        )),
    }
}

macro_rules! rel_check_op {
    ($a:expr, $a_rng:expr, $op:tt, $b:expr, $b_rng:expr, $file_id:expr $(, $($x:pat => $y:expr),*)?) => {
        match (
            $a.get_value().get_type_id_and_value(),
            $b.get_value().get_type_id_and_value(),
        ) {
            (TypeIdAndValue::Error, tright) => Err(OpsError::new(
                    OpsErrorType::IncompatibleError,
                    Diagnostic::error()
                        .with_message("Incompatible operands(one is an error)")
                        .with_labels(vec![
                            Label::primary($file_id, $a_rng)
                                .with_message(TypeId::Error.typename()),
                            Label::secondary($file_id, $b_rng)
                                .with_message(tright.degrade().typename()),
                        ]),
            )),
            (tleft, TypeIdAndValue::Error) => Err(OpsError::new(
                    OpsErrorType::IncompatibleError,
                    Diagnostic::error()
                        .with_message("Incompatible operands(one is an error)")
                        .with_labels(vec![
                            Label::primary($file_id, $a_rng)
                                .with_message(tleft.degrade().typename()),
                            Label::secondary($file_id, $b_rng)
                                .with_message(TypeId::Error.typename()),
                        ]),
            )),
            $($($x => $y),*,)?
            (left, right) if left $op right => Ok(Value::new(Box::new(true))),
            (_tleft, _tright) => Ok(Value::new(Box::new(false))),
        }
    };
}

pub(crate) fn op_eq(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Range<usize>,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Range<usize>,
    file_id: usize,
) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
    rel_check_op!(ls, left_range, ==, rs, right_range, file_id)
}

pub(crate) fn op_neq(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Range<usize>,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Range<usize>,
    file_id: usize,
) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
    rel_check_op!(ls, left_range, !=, rs, right_range, file_id)
}

pub(crate) fn op_l(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Range<usize>,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Range<usize>,
    file_id: usize,
) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
    rel_check_op!(ls, left_range, <, rs, right_range, file_id)
}

pub(crate) fn op_g(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Range<usize>,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Range<usize>,
    file_id: usize,
) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
    rel_check_op!(ls, left_range, >, rs, right_range, file_id)
}

pub(crate) fn op_le(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Range<usize>,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Range<usize>,
    file_id: usize,
) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
    rel_check_op!(ls, left_range, <=, rs, right_range, file_id)
}

pub(crate) fn op_ge(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Range<usize>,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Range<usize>,
    file_id: usize,
) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
    rel_check_op!(ls, left_range, >=, rs, right_range, file_id)
}

pub(crate) fn op_unary_not(
    value: &Value<Box<dyn ValueTypeMarker>>,
    range: Range<usize>,
    file_id: usize,
) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
    match value.get_value().get_type_id_and_value() {
        TypeIdAndValue::Error => Err(OpsError::new(
            OpsErrorType::IncompatibleError,
            Diagnostic::error()
                .with_message("Incompatible operands(one is an error)")
                .with_labels(vec![
                    Label::primary(file_id, range).with_message(TypeId::Error.typename())
                ]),
        )),
        TypeIdAndValue::Bool(x) => Ok(Value::new(Box::new(!*x))),
        other => Err(OpsError::new(
            OpsErrorType::Incompatible,
            Diagnostic::error()
                .with_message("Cannot negate a non-boolean value")
                .with_labels(vec![
                    Label::primary(file_id, range).with_message(other.degrade().typename())
                ]),
        )),
    }
}

pub(crate) fn op_unary_neg(
    value: &Value<Box<dyn ValueTypeMarker>>,
    range: Range<usize>,
    file_id: usize,
) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
    match value.get_value().get_type_id_and_value() {
        TypeIdAndValue::I32(x) => Ok(Value::new(Box::new(-*x))),
        TypeIdAndValue::I64(x) => Ok(Value::new(Box::new(-*x))),
        TypeIdAndValue::Error => Err(OpsError::new(
            OpsErrorType::IncompatibleError,
            Diagnostic::error()
                .with_message("Incompatible operands(one is an error)")
                .with_labels(vec![
                    Label::primary(file_id, range).with_message(TypeId::Error.typename())
                ]),
        )),
        other => Err(OpsError::new(
            OpsErrorType::Incompatible,
            Diagnostic::error()
                .with_message("Incompatible operands")
                .with_labels(vec![
                    Label::primary(file_id, range).with_message(other.degrade().typename())
                ]),
        )),
    }
}
