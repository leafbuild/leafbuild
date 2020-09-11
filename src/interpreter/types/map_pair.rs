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
        TypeId::MapPair
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue {
        TypeIdAndValue::MapPair(self)
    }
}

pub(crate) fn get_map_pair_call_pool() -> CallPool {
    CallPool::new(vec![])
}

pub(crate) fn resolve_map_pair_property_access(
    base: Value<Box<dyn ValueTypeMarker>>,
    base_location: Location,
    name: &str,
    name_location: TokLoc,
    frame: &EnvFrame,
) -> Value<Box<dyn ValueTypeMarker>> {
    match name {
        "key" => base
            .get_type_id_and_value_required(TypeId::MapPair)
            .unwrap()
            .get_map_pair()
            .unwrap()
            .key
            .clone_to_value(),
        "value" => base
            .get_type_id_and_value_required(TypeId::MapPair)
            .unwrap()
            .get_map_pair()
            .unwrap()
            .value
            .clone_to_value(),
        _ => {
            push_diagnostic(
                UnknownPropertyError::new(
                    ExprLocAndType::new(base_location, TypeId::MapPair.typename()),
                    name,
                    name_location.as_rng(),
                ),
                frame,
            );
            Value::new(Box::new(ErrorValue::new()))
        }
    }
}
