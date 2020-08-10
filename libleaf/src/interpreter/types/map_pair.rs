pub(crate) struct MapPair {
    key: String,
    value: Value<Box<dyn ValueTypeMarker>>,
}

impl MapPair {
    pub(crate) fn new(key: String, value: Value<Box<dyn ValueTypeMarker>>) -> MapPair {
        Self { key, value }
    }
}

impl ValueTypeMarker for MapPair {
    fn stringify(&self) -> String {
        format!(
            "map pair {{ key = '{}', value = {} }}",
            self.key,
            self.value.stringify()
        )
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(MapPair::new(
            self.key.clone(),
            self.value.clone_to_value(),
        )))
    }

    fn get_type_id(&self) -> TypeId {
        unimplemented!()
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue {
        unimplemented!()
    }
}
