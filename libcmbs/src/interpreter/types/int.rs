impl ValueTypeMarker for i32 {
    fn stringify(&self) -> String {
        format!("{}", self)
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(*self))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::I32(self)
    }

    fn get_func_call_pool(&self) -> FuncCallPool {
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
        TypeId::I64(self)
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
        TypeId::U32(self)
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
        TypeId::U64(self)
    }
}

#[inline]
pub(crate) fn get_num_call_pool() -> FuncCallPool {
    FuncCallPool::new(vec![FuncCallExecutor::new(
        "to_string".to_string(),
        |args, frame, base| Value::new(Box::new(base.unwrap().get_value().deref().stringify())),
    )])
}
