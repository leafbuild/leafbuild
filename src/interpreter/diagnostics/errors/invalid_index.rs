pub(crate) struct InvalidIndexError {
    base_expr: ExprLocAndType,
    index_expr: ExprLocAndType,
}

impl InvalidIndexError {
    pub(crate) fn new(base_expr: ExprLocAndType, index_expr: ExprLocAndType) -> Self {
        Self {
            base_expr,
            index_expr,
        }
    }
}

impl LeafDiagnosticTrait for InvalidIndexError {
    fn get_diagnostic(&self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        LeafDiagnostic::error()
            .with_message("Invalid index for indexed expression")
            .with_code(INVALID_INDEX_ERROR)
            .with_labels(vec![
                LeafLabel::primary(ctx.get_current_file(), self.base_expr.loc.clone())
                    .with_message(self.base_expr.type_.clone()),
                LeafLabel::secondary(ctx.get_current_file(), self.index_expr.loc.clone())
                    .with_message(self.index_expr.type_.clone()),
            ])
    }

    fn should_print(&self, _: &DiagnosticsCtx) -> bool {
        true
    }
}
