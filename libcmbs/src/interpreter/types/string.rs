impl ValueTypeMarker for String {
    fn stringify(&self) -> String {
        format!("{}", self)
    }
    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(self.clone()))
    }
}
