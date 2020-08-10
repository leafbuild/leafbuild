#[derive(Clone)]
pub(crate) struct Executable {
    name: String,
    id: usize,
    sources: Vec<String>,
}

impl Executable {
    pub(crate) fn new(id: usize, name: String, sources: Vec<String>) -> Self {
        Self { id, name, sources }
    }

    pub(crate) fn make_ref(&self) -> ExeRef {
        ExeRef::new(self.id)
    }

    pub(crate) fn get_name(&self) -> &String {
        &self.name
    }
}

pub(crate) struct ExeRef {
    id: usize,
}

impl ExeRef {
    fn new(id: usize) -> Self {
        Self { id }
    }
}

impl ValueTypeMarker for ExeRef {
    fn stringify(&self) -> String {
        format!("executable reference {{ id = '{}' }}", self.id)
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(ExeRef { id: self.id }))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::ExecutableReference
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue {
        TypeIdAndValue::ExecutableReference(self)
    }
}

pub(crate) fn get_executable_call_pool() -> CallPool {
    CallPool::new(vec![])
}

pub(crate) fn resolve_executable_property_access(
    base: Value<Box<dyn ValueTypeMarker>>,
    property_name: &str,
) -> Value<Box<dyn ValueTypeMarker>> {
    match property_name {
        _ => panic!("Unknown property on executable: {}", property_name),
    }
}
