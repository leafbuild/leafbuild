impl ValueTypeMarker for bool {
    fn stringify(&self) -> String {
        format!("{}", self)
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(*self))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::Bool
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue {
        TypeIdAndValue::Bool(self)
    }
}

pub(crate) fn get_bool_call_pool() -> CallPool {
    CallPool::new(vec![])
}
