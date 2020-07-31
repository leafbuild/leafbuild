impl ValueTypeMarker for () {
    fn stringify(&self) -> String {
        "(void)".to_string()
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(()))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::Void
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue<'_> {
        TypeIdAndValue::Void
    }
}
