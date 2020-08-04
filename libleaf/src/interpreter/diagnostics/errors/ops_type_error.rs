/// Returned when operands to an operation are invalid.
pub(crate) struct OpsTypeError {
    /// is None for unary operations.
    left: Option<ExprLocAndType>,
    /// as it is sometimes not evaluated, as it is the case with the `and` and `or` logic operators,
    /// it will have a "not-evaluated" type
    right: ExprLocAndType,
}

impl OpsTypeError {
    pub(crate) fn new(left: ExprLocAndType, right: ExprLocAndType) -> Self {
        Self {
            left: Some(left),
            right,
        }
    }

    pub(crate) fn new_unary(right: ExprLocAndType) -> Self {
        Self { left: None, right }
    }
}

impl LeafDiagnosticTrait for OpsTypeError {
    fn get_diagnostic(self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        LeafDiagnostic::error()
            .with_message(format!(
                "Incompatible operand{} for operation",
                if self.left.is_none() { "" } else { "s" }
            ))
            .with_code(OPS_TYPE_ERROR)
            .with_labels(match self.left {
                Some(left) => vec![
                    LeafLabel::primary(ctx.current_file, left.loc).with_message(left.type_),
                    LeafLabel::primary(ctx.current_file, self.right.loc)
                        .with_message(self.right.type_),
                ],
                None => vec![LeafLabel::primary(ctx.current_file, self.right.loc)
                    .with_message(self.right.type_)],
            })
    }

    fn should_print<'env>(&self, _ctx: &DiagnosticsCtx) -> bool {
        true
    }
}

/// Returned when at least one operand to an operation is of [the `error` type](https://leafbuild.gitlab.io/docs/reference/special_types/error.html).
pub(crate) struct OpsTypeErrorError {
    error_expr_loc: Location,
    other_expr: Option<ExprLocAndType>,
}

impl OpsTypeErrorError {
    pub(crate) fn new(error_expr_loc: Location, other_expr: Option<ExprLocAndType>) -> Self {
        Self {
            error_expr_loc,
            other_expr,
        }
    }
}

impl LeafDiagnosticTrait for OpsTypeErrorError {
    fn get_diagnostic(self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        LeafDiagnostic::error()
            .with_message("Incompatible operands for operation")
            .with_code(OPS_TYPE_ERROR_ERROR)
            .with_labels(match self.other_expr {
                Some(other) => vec![
                    LeafLabel::primary(ctx.current_file, other.loc).with_message(other.type_),
                    LeafLabel::primary(ctx.current_file, self.error_expr_loc)
                        .with_message(TypeId::Error.typename()),
                ],
                None => vec![LeafLabel::primary(ctx.current_file, self.error_expr_loc)
                    .with_message(TypeId::Error.typename())],
            })
    }

    fn should_print(&self, ctx: &DiagnosticsCtx) -> bool {
        ctx.get_error_cascade()
    }
}
