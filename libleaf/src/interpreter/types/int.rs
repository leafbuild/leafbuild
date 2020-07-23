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

    fn get_func_call_pool(&self) -> CallPool {
        get_global_functions()
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
        |_args, _frame, base: Option<&Value<Box<dyn ValueTypeMarker>>>| {
            Value::new(Box::new(base.unwrap().get_value().deref().stringify()))
        },
    )])
}

pub(crate) fn resolve_num_property_access(
    num: &Value<Box<dyn ValueTypeMarker>>,
    property_name: &str,
) -> Value<Box<dyn ValueTypeMarker>> {
    match property_name as &str {
        "as_str" => match num.get_value().get_type_id() {
            TypeId::I32 | TypeId::I64 | TypeId::U32 | TypeId::U64 => {
                Value::new(Box::new(num.get_value().stringify()))
            }
            _ => panic!("Not an int"),
        },
        _ => panic!("Unknown property on number: {}", property_name),
    }
}
