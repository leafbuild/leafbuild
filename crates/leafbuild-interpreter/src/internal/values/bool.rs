#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub struct BoolWrap(pub bool);

impl<'frame> Value<'frame> for BoolWrap {
    fn get_type(&self) -> ValueType {
        ValueType::Bool
    }

    fn get_property(
        &self,
        this_span: Span,
        dot_span: Span,
        property_name: &str,
        property_name_span: Span,
    ) -> Result<&'frame dyn Value<'frame>, GetPropertyError> {
        unimplemented!()
    }

    fn get_property_mut(
        &mut self,
        this_span: Span,
        dot_span: Span,
        property_name: &str,
        property_name_span: Span,
    ) -> Result<&'frame mut dyn Value<'frame>, GetPropertyError> {
        unimplemented!()
    }

    fn get_indexed(
        &self,
        this_span: Span,
        left_brace: Span,
        index_value: &dyn Value,
        right_brace: Span,
    ) -> Result<&'frame dyn Value<'frame>, GetIndexedError> {
        unimplemented!()
    }

    fn get_indexed_mut(
        &mut self,
        this_span: Span,
        left_brace: Span,
        index_value: &dyn Value,
        right_brace: Span,
    ) -> Result<&'frame mut dyn Value<'frame>, GetIndexedError> {
        unimplemented!()
    }

    fn invoke_method(
        &self,
        this_span: Span,
        dot_span: Span,
        method_name: &str,
        method_name_span: Span,
    ) -> Result<Box<dyn Value<'static>>, InvokeMethodError> {
        unimplemented!()
    }
}
