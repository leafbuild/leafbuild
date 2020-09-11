pub(crate) struct IndexOutsideVectorError {
    base_expr: Location,
    index_expr: Location,
    vec_len: usize,
    index_val: usize,
}

impl IndexOutsideVectorError {
    pub(crate) fn new(
        base_expr: Location,
        index_expr: Location,
        vec_len: usize,
        index_val: usize,
    ) -> Self {
        Self {
            base_expr,
            index_expr,
            vec_len,
            index_val,
        }
    }
}

impl LeafDiagnosticTrait for IndexOutsideVectorError {
    fn get_diagnostic(&self, ctx: &DiagnosticsCtx) -> LeafDiagnostic {
        LeafDiagnostic::error()
            .with_message("Index outside vector")
            .with_code(INDEX_OUTSIDE_VECTOR_ERROR)
            .with_labels(vec![
                LeafLabel::primary(ctx.get_current_file(), self.base_expr.clone()).with_message(
                    format!("{} of length {}", TypeId::Vec.typename(), self.vec_len,),
                ),
                LeafLabel::secondary(ctx.get_current_file(), self.index_expr.clone())
                    .with_message(format!("{}", self.index_val)),
            ])
    }

    fn should_print(&self, _: &DiagnosticsCtx) -> bool {
        true
    }
}
