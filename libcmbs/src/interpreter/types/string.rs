impl ValueTypeMarker for String {
    fn stringify(&self) -> String {
        format!("'{}'", self)
    }
    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(self.clone()))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::String(self)
    }
}

#[inline]
pub(crate) fn get_string_call_pool() -> FuncCallPool {
    FuncCallPool::new(vec![FuncCallExecutor::new(
        "to_string".to_string(),
        |args, frame, base| Value::new(Box::new(base.unwrap().get_value().deref().stringify())),
    )])
}
