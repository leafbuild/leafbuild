impl ValueTypeMarker for Vec<Value<Box<dyn ValueTypeMarker>>> {
    fn stringify(&self) -> String {
        format!("[{}]", self.iter().map(|v| { v.stringify() }).join(", "))
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        let mut cl: Vec<Value<Box<dyn ValueTypeMarker>>> = Vec::with_capacity(self.capacity());
        self.iter().for_each(|v| cl.push(v.clone_to_value()));
        Value::new(Box::new(cl))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::Vec
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue<'_> {
        TypeIdAndValue::Vec(self)
    }
}

pub(crate) fn get_vec_call_pool() -> CallPool {
    CallPool::new(vec![])
}

pub(crate) fn resolve_vec_property_access(
    _v: &Value<Box<dyn ValueTypeMarker>>,
    property_name: &str,
) -> Value<Box<dyn ValueTypeMarker>> {
    match property_name as &str {
        _ => panic!("Unknown property on vec: {}", property_name),
    }
}
