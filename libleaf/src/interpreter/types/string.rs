impl ValueTypeMarker for String {
    fn stringify(&self) -> String {
        format!("'{}'", self)
    }
    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(self.clone()))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::String
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue {
        TypeIdAndValue::String(self)
    }
}

#[inline]
pub(crate) fn get_string_call_pool() -> CallPool {
    CallPool::new(vec![CallExecutor::new(
        "to_string".to_string(),
        |_args, _frame, base| Value::new(Box::new(base.unwrap().get_value().deref().stringify())),
    )])
}

pub(crate) fn resolve_str_property_access(
    _string: &Value<Box<dyn ValueTypeMarker>>,
    property_name: &str,
) -> Value<Box<dyn ValueTypeMarker>> {
    match property_name as &str {
        _ => panic!("Unknown property on string: {}", property_name),
    }
}
