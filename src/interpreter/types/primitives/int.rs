impl ValueTypeMarker for i32 {
    fn stringify(&self) -> String {
        format!("{}", self)
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(*self))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::I32
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue {
        TypeIdAndValue::I32(self)
    }
}

impl ValueTypeMarker for i64 {
    fn stringify(&self) -> String {
        format!("{}", self)
    }
    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(*self))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::I64
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue {
        TypeIdAndValue::I64(self)
    }
}

impl ValueTypeMarker for u32 {
    fn stringify(&self) -> String {
        format!("{}", self)
    }
    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(*self))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::U32
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue {
        TypeIdAndValue::U32(self)
    }
}

impl ValueTypeMarker for u64 {
    fn stringify(&self) -> String {
        format!("{}", self)
    }
    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(*self))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::U64
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue {
        TypeIdAndValue::U64(self)
    }
}

#[inline]
pub(crate) fn get_num_call_pool() -> CallPool {
    CallPool::new(vec![CallExecutor::new(
        "to_string".to_string(),
        |_loc, _args, _frame, base: Option<&Value<Box<dyn ValueTypeMarker>>>| {
            Value::new(Box::new(base.unwrap().get_value().deref().stringify()))
        },
        vec![],
    )])
}

pub(crate) fn resolve_num_property_access(
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
                    ExprLocAndType::new(base_location, base.get_type_id().typename()),
                    name,
                    name_location.as_rng(),
                ),
                frame,
            );
            Value::new(Box::new(ErrorValue::new()))
        }
    }
}
