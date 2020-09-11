pub(crate) struct ErrorValue {}

impl ErrorValue {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl ValueTypeMarker for ErrorValue {
    fn stringify(&self) -> String {
        "(error)".to_string()
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(Self::new()))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::Error
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue<'_> {
        TypeIdAndValue::Error
    }
}

pub(crate) fn get_error_call_pool() -> CallPool {
    CallPool::new(vec![])
}
