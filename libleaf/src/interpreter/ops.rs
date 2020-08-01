use crate::interpreter::types::TypeId;
use crate::interpreter::{types::TypeIdAndValue, Value, ValueTypeMarker};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use std::ops::Range;

#[derive(Copy, Clone)]
pub(crate) enum OpsErrorType {
    Incompatible,
}

pub(crate) struct OpsError {
    type_: OpsErrorType,
    diagnostic: Diagnostic<usize>,
}

impl OpsError {
    fn new(type_: OpsErrorType, diagnostic: Diagnostic<usize>) -> OpsError {
        Self { type_, diagnostic }
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
