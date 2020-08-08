use crate::interpreter::{
    diagnostics::{
        self,
        errors::{self, ExprLocAndType},
        DiagnosticsCtx, Location,
    },
    types::{ErrorValue, TypeId, TypeIdAndValue},
    EnvFrame, LaterValue, Value, ValueTypeMarker,
};

macro_rules! op_match {
    ($a:expr, $a_rng:expr, $op:tt, $b:expr, $b_rng:expr, $ctx:expr $(, $($x:pat => $y:expr),*)?) => {
        match (
            $a.get_value().get_type_id_and_value(),
            $b.get_value().get_type_id_and_value(),
        ) {
            (TypeIdAndValue::Error, tright) => {
                diagnostics::push_diagnostic_ctx(errors::OpsTypeErrorError::new(
                        $a_rng, Some(ExprLocAndType::new($b_rng, tright.degrade().typename()))
                ), $ctx);
                Value::new(Box::new(ErrorValue::new()))
            },
            (tleft, TypeIdAndValue::Error) => {
                diagnostics::push_diagnostic_ctx(errors::OpsTypeErrorError::new(
                        $b_rng, Some(ExprLocAndType::new($a_rng, tleft.degrade().typename()))
                ), $ctx);
                Value::new(Box::new(ErrorValue::new()))
            },
            $($($x => $y),*,)?
            (TypeIdAndValue::I32(leftval), tright) => match tright {
                TypeIdAndValue::I32(rightval) => Value::new(Box::new(*leftval $op *rightval)),
                TypeIdAndValue::I64(rightval) => {
                    Value::new(Box::new(*leftval as i64 $op *rightval))
                }
                _ => {
                    diagnostics::push_diagnostic_ctx(errors::OpsTypeError::new(
                            ExprLocAndType::new($a_rng, TypeId::I32.typename()), ExprLocAndType::new($b_rng, tright.degrade().typename())
                    ), $ctx);
                    Value::new(Box::new(ErrorValue::new()))
                },
            },
            (TypeIdAndValue::I64(leftval), tright) => match tright {
                TypeIdAndValue::I32(rightval) => {
                    Value::new(Box::new(*leftval $op (*rightval as i64)))
                }
                TypeIdAndValue::I64(rightval) => Value::new(Box::new(*leftval $op *rightval)),
                _ => {
                    diagnostics::push_diagnostic_ctx(errors::OpsTypeError::new(
                            ExprLocAndType::new($a_rng, TypeId::I64.typename()), ExprLocAndType::new($b_rng, tright.degrade().typename())
                    ), $ctx);
                    Value::new(Box::new(ErrorValue::new()))
                },
            },
            (TypeIdAndValue::U32(leftval), tright) => match tright {
                TypeIdAndValue::U32(rightval) => Value::new(Box::new(*leftval $op *rightval)),
                TypeIdAndValue::U64(rightval) => {
                    Value::new(Box::new(*leftval as u64 $op *rightval))
                }
                _ => {
                    diagnostics::push_diagnostic_ctx(errors::OpsTypeError::new(
                            ExprLocAndType::new($a_rng, TypeId::U32.typename()), ExprLocAndType::new($b_rng, tright.degrade().typename())
                    ), $ctx);
                    Value::new(Box::new(ErrorValue::new()))
                },
            },
            (TypeIdAndValue::U64(leftval), tright) => match tright {
                TypeIdAndValue::U32(rightval) => {
                    Value::new(Box::new(*leftval $op (*rightval as u64)))
                }
                TypeIdAndValue::U64(rightval) => Value::new(Box::new(*leftval $op *rightval)),
                _ => {
                    diagnostics::push_diagnostic_ctx(errors::OpsTypeError::new(
                            ExprLocAndType::new($a_rng, TypeId::U64.typename()), ExprLocAndType::new($b_rng, tright.degrade().typename())
                    ), $ctx);
                    Value::new(Box::new(ErrorValue::new()))
                },
            },
            (tleft, tright) => {
                    diagnostics::push_diagnostic_ctx(errors::OpsTypeError::new(
                            ExprLocAndType::new($a_rng, tleft.degrade().typename()), ExprLocAndType::new($b_rng, tright.degrade().typename())
                    ), $ctx);
                    Value::new(Box::new(ErrorValue::new()))
                },
        }
    };
}

pub(crate) fn op_add(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Location,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Location,
    ctx: &DiagnosticsCtx,
) -> Value<Box<dyn ValueTypeMarker>> {
    op_match!(ls, left_range, +, rs, right_range, ctx,
    (TypeIdAndValue::String(left), tright) => Value::new(Box::new(format!(
        "{}{}",
        left,
        tright.stringify()
    ))),
    (tleft, TypeIdAndValue::String(right)) => Value::new(Box::new(format!(
        "{}{}",
        tleft.stringify(),
        right
    ))))
}

pub(crate) fn op_sub(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Location,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Location,
    ctx: &DiagnosticsCtx,
) -> Value<Box<dyn ValueTypeMarker>> {
    op_match!(ls, left_range, -, rs, right_range, ctx)
}
pub(crate) fn op_mul(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Location,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Location,
    ctx: &DiagnosticsCtx,
) -> Value<Box<dyn ValueTypeMarker>> {
    op_match!(ls, left_range, *, rs, right_range, ctx)
}
pub(crate) fn op_div(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Location,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Location,
    ctx: &DiagnosticsCtx,
) -> Value<Box<dyn ValueTypeMarker>> {
    op_match!(ls, left_range, /, rs, right_range, ctx)
}
pub(crate) fn op_mod(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Location,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Location,
    ctx: &DiagnosticsCtx,
) -> Value<Box<dyn ValueTypeMarker>> {
    op_match!(ls, left_range, %, rs, right_range, ctx)
}

pub(crate) fn op_and<'a>(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Location,
    rs: &LaterValue<'a>,
    right_range: Location,
    frame: &mut EnvFrame,
) -> Value<Box<dyn ValueTypeMarker>> {
    match ls.get_value().get_type_id_and_value() {
        TypeIdAndValue::Error => {
            diagnostics::push_diagnostic_ctx(
                errors::OpsTypeErrorError::new(left_range, None),
                frame.get_diagnostics_ctx(),
            );
            Value::new(Box::new(ErrorValue::new()))
        }
        TypeIdAndValue::Bool(left) => {
            if !left {
                return Value::new(Box::new(false));
            }
            match rs.compute(frame).get_value().get_type_id_and_value() {
                TypeIdAndValue::Error => {
                    diagnostics::push_diagnostic_ctx(
                        errors::OpsTypeErrorError::new(
                            right_range,
                            Some(ExprLocAndType::new(left_range, TypeId::Bool.typename())),
                        ),
                        frame.get_diagnostics_ctx(),
                    );
                    Value::new(Box::new(ErrorValue::new()))
                }
                TypeIdAndValue::Bool(right) => Value::new(Box::new(*right)),
                tright => {
                    diagnostics::push_diagnostic_ctx(
                        errors::OpsTypeError::new(
                            ExprLocAndType::new(left_range, TypeId::Bool.typename()),
                            ExprLocAndType::new(right_range, tright.degrade().typename()),
                        ),
                        frame.get_diagnostics_ctx(),
                    );
                    Value::new(Box::new(ErrorValue::new()))
                }
            }
        }
        tleft => {
            diagnostics::push_diagnostic_ctx(
                errors::OpsTypeError::new(
                    ExprLocAndType::new(left_range, tleft.degrade().typename()),
                    ExprLocAndType::new(right_range, "not evaluated"),
                ),
                frame.get_diagnostics_ctx(),
            );
            Value::new(Box::new(ErrorValue::new()))
        }
    }
}

pub(crate) fn op_or<'a>(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Location,
    rs: &LaterValue<'a>,
    right_range: Location,
    frame: &mut EnvFrame,
) -> Value<Box<dyn ValueTypeMarker>> {
    match ls.get_value().get_type_id_and_value() {
        TypeIdAndValue::Error => {
            diagnostics::push_diagnostic_ctx(
                errors::OpsTypeErrorError::new(left_range, None),
                frame.get_diagnostics_ctx(),
            );
            Value::new(Box::new(ErrorValue::new()))
        }
        TypeIdAndValue::Bool(left) => {
            if *left {
                return Value::new(Box::new(true));
            }
            match rs.compute(frame).get_value().get_type_id_and_value() {
                TypeIdAndValue::Error => {
                    diagnostics::push_diagnostic_ctx(
                        errors::OpsTypeErrorError::new(
                            right_range,
                            Some(ExprLocAndType::new(left_range, TypeId::Bool.typename())),
                        ),
                        frame.get_diagnostics_ctx(),
                    );
                    Value::new(Box::new(ErrorValue::new()))
                }
                TypeIdAndValue::Bool(right) => Value::new(Box::new(*right)),
                tright => {
                    diagnostics::push_diagnostic_ctx(
                        errors::OpsTypeError::new(
                            ExprLocAndType::new(left_range, TypeId::Bool.typename()),
                            ExprLocAndType::new(right_range, tright.degrade().typename()),
                        ),
                        frame.get_diagnostics_ctx(),
                    );
                    Value::new(Box::new(ErrorValue::new()))
                }
            }
        }
        tleft => {
            diagnostics::push_diagnostic_ctx(
                errors::OpsTypeError::new(
                    ExprLocAndType::new(left_range, tleft.degrade().typename()),
                    ExprLocAndType::new(right_range, "not evaluated"),
                ),
                frame.get_diagnostics_ctx(),
            );
            Value::new(Box::new(ErrorValue::new()))
        }
    }
}

macro_rules! rel_check_op {
    ($a:expr, $a_rng:expr, $op:tt, $b:expr, $b_rng:expr, $ctx:expr $(, $($x:pat => $y:expr),*)?) => {
        match (
            $a.get_value().get_type_id_and_value(),
            $b.get_value().get_type_id_and_value(),
        ) {
            (TypeIdAndValue::Error, tright) => {
                diagnostics::push_diagnostic_ctx(errors::OpsTypeErrorError::new(
                        $a_rng, Some(ExprLocAndType::new($b_rng, tright.degrade().typename()))
                ), $ctx);
                Value::new(Box::new(ErrorValue::new()))
            },
            (tleft, TypeIdAndValue::Error) => {
                diagnostics::push_diagnostic_ctx(errors::OpsTypeErrorError::new(
                        $b_rng, Some(ExprLocAndType::new($a_rng, tleft.degrade().typename()))
                ), $ctx);
                Value::new(Box::new(ErrorValue::new()))
            },
            $($($x => $y),*,)?
            (TypeIdAndValue::I32(a), TypeIdAndValue::I32(b)) => Value::new(Box::new((*a) $op (*b))),
            (TypeIdAndValue::I32(a), TypeIdAndValue::I64(b)) => Value::new(Box::new((*a as i64) $op (*b))),
            (TypeIdAndValue::I64(a), TypeIdAndValue::I32(b)) => Value::new(Box::new((*a) $op (*b as i64))),
            (TypeIdAndValue::I64(a), TypeIdAndValue::I64(b)) => Value::new(Box::new((*a) $op (*b))),

            (TypeIdAndValue::U32(a), TypeIdAndValue::U32(b)) => Value::new(Box::new((*a) $op (*b))),
            (TypeIdAndValue::U32(a), TypeIdAndValue::U64(b)) => Value::new(Box::new((*a as u64) $op (*b))),
            (TypeIdAndValue::U64(a), TypeIdAndValue::U32(b)) => Value::new(Box::new((*a) $op (*b as u64))),
            (TypeIdAndValue::U64(a), TypeIdAndValue::U64(b)) => Value::new(Box::new((*a) $op (*b))),

            (tleft, tright) => {
                    diagnostics::push_diagnostic_ctx(errors::OpsTypeError::new(
                            ExprLocAndType::new($a_rng, tleft.degrade().typename()), ExprLocAndType::new($b_rng, tright.degrade().typename())
                    ), $ctx);
                    Value::new(Box::new(ErrorValue::new()))
            }
        }
    };
}

macro_rules! rel_check_op_allow_boolean {
    ($a:expr, $a_rng:expr, $op:tt, $b:expr, $b_rng:expr, $ctx:expr $(, $($x:pat => $y:expr),*)?) => {
    rel_check_op!($a, $a_rng, $op, $b, $b_rng, $ctx, (TypeIdAndValue::Bool(a), TypeIdAndValue::Bool(b)) => Value::new(Box::new((*a) $op (*b))) $(, $($x => $y)*)?)
    };
}

pub(crate) fn op_eq(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Location,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Location,
    ctx: &DiagnosticsCtx,
) -> Value<Box<dyn ValueTypeMarker>> {
    rel_check_op_allow_boolean!(ls, left_range, ==, rs, right_range, ctx)
}

pub(crate) fn op_neq(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Location,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Location,
    ctx: &DiagnosticsCtx,
) -> Value<Box<dyn ValueTypeMarker>> {
    rel_check_op_allow_boolean!(ls, left_range, !=, rs, right_range, ctx)
}

pub(crate) fn op_l(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Location,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Location,
    ctx: &DiagnosticsCtx,
) -> Value<Box<dyn ValueTypeMarker>> {
    rel_check_op!(ls, left_range, <, rs, right_range, ctx)
}

pub(crate) fn op_g(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Location,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Location,
    ctx: &DiagnosticsCtx,
) -> Value<Box<dyn ValueTypeMarker>> {
    rel_check_op!(ls, left_range, >, rs, right_range, ctx)
}

pub(crate) fn op_le(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Location,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Location,
    ctx: &DiagnosticsCtx,
) -> Value<Box<dyn ValueTypeMarker>> {
    rel_check_op!(ls, left_range, <=, rs, right_range, ctx)
}

pub(crate) fn op_ge(
    ls: &Value<Box<dyn ValueTypeMarker>>,
    left_range: Location,
    rs: &Value<Box<dyn ValueTypeMarker>>,
    right_range: Location,
    ctx: &DiagnosticsCtx,
) -> Value<Box<dyn ValueTypeMarker>> {
    rel_check_op!(ls, left_range, >=, rs, right_range, ctx)
}

pub(crate) fn op_unary_not(
    value: &Value<Box<dyn ValueTypeMarker>>,
    range: Location,
    ctx: &DiagnosticsCtx,
) -> Value<Box<dyn ValueTypeMarker>> {
    match value.get_value().get_type_id_and_value() {
        TypeIdAndValue::Error => {
            diagnostics::push_diagnostic_ctx(errors::OpsTypeErrorError::new(range, None), ctx);
            Value::new(Box::new(ErrorValue::new()))
        }
        TypeIdAndValue::Bool(x) => Value::new(Box::new(!*x)),
        other => {
            diagnostics::push_diagnostic_ctx(
                errors::OpsTypeError::new_unary(ExprLocAndType::new(
                    range,
                    other.degrade().typename(),
                )),
                ctx,
            );
            Value::new(Box::new(ErrorValue::new()))
        }
    }
}

pub(crate) fn op_unary_neg(
    value: &Value<Box<dyn ValueTypeMarker>>,
    range: Location,
    ctx: &DiagnosticsCtx,
) -> Value<Box<dyn ValueTypeMarker>> {
    match value.get_value().get_type_id_and_value() {
        TypeIdAndValue::Error => {
            diagnostics::push_diagnostic_ctx(errors::OpsTypeErrorError::new(range, None), ctx);
            Value::new(Box::new(ErrorValue::new()))
        }
        TypeIdAndValue::I32(x) => Value::new(Box::new(-*x)),
        TypeIdAndValue::I64(x) => Value::new(Box::new(-*x)),
        other => {
            diagnostics::push_diagnostic_ctx(
                errors::OpsTypeError::new_unary(ExprLocAndType::new(
                    range,
                    other.degrade().typename(),
                )),
                ctx,
            );
            Value::new(Box::new(ErrorValue::new()))
        }
    }
}
