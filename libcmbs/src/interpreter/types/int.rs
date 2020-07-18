impl ValueTypeMarker for i32 {
    fn stringify(&self) -> String {
        format!("{}", self)
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(*self))
    }
}

impl ValueTypeMarker for i64 {
    fn stringify(&self) -> String {
        format!("{}", self)
    }
    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(*self))
    }
}

impl ValueTypeMarker for u32 {
    fn stringify(&self) -> String {
        format!("{}", self)
    }
    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(*self))
    }
}

impl ValueTypeMarker for u64 {
    fn stringify(&self) -> String {
        format!("{}", self)
    }
    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(*self))
    }
}
