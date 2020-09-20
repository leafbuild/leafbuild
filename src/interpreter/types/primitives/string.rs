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

pub(crate) fn get_string_call_pool() -> CallPool {
    CallPool::new(vec![CallExecutor::new(
        "to_string".to_string(),
        |_loc, _args, _frame, base| {
            Value::new(Box::new(base.unwrap().get_value().deref().stringify()))
        },
        vec![],
    )])
}

pub(crate) fn resolve_str_property_access(
    base: Value<Box<dyn ValueTypeMarker>>,
    base_location: Location,
    name: &str,
    name_location: TokLoc,
    frame: &EnvFrame,
) -> Value<Box<dyn ValueTypeMarker>> {
    match name {
        _ => {
            push_diagnostic(
                UnknownPropertyError::new(
                    ExprLocAndType::new(base_location, TypeId::String.typename()),
                    name,
                    name_location.as_rng(),
                ),
                frame,
            );
            Value::new(Box::new(ErrorValue::new()))
        }
    }
}
