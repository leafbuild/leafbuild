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
    base: Value<Box<dyn ValueTypeMarker>>,
    base_location: Location,
    name: &str,
    name_location: TokLoc,
    frame: &EnvFrame,
) -> Value<Box<dyn ValueTypeMarker>> {
    match name as &str {
        _ => {
            push_diagnostic(
                UnknownPropertyError::new(
                    ExprLocAndType::new(base_location, TypeId::Vec.typename()),
                    name,
                    name_location.as_rng(),
                ),
                frame,
            );
            Value::new(Box::new(ErrorValue::new()))
        }
    }
}
