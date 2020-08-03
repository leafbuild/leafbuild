impl ValueTypeMarker for HashMap<String, Value<Box<dyn ValueTypeMarker>>> {
    fn stringify(&self) -> String {
        format!(
            "{{{}}}",
            self.iter()
                .map(|(k, v)| { format!("{}: {}", k, v.stringify()) })
                .join(", ")
        )
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        let mut cl: HashMap<String, Value<Box<dyn ValueTypeMarker>>> =
            HashMap::with_capacity(self.capacity());
        self.iter().for_each(|(k, v)| {
            cl.insert(k.clone(), v.clone_to_value());
        });
        Value::new(Box::new(cl))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::Vec
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue<'_> {
        TypeIdAndValue::Map(self)
    }
}

pub(crate) fn get_map_call_pool() -> CallPool {
    CallPool::new(vec![])
}

pub(crate) fn resolve_map_property_access(
    _v: &Value<Box<dyn ValueTypeMarker>>,
    property_name: &str,
) -> Value<Box<dyn ValueTypeMarker>> {
    match property_name as &str {
        _ => panic!("Unknown property on map: {}", property_name),
    }
}
