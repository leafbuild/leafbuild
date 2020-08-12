#[derive(Clone)]
pub(crate) struct Executable {
    name: String,
    id: usize,
    sources: Vec<String>,
    include_dirs: Vec<String>,
}

impl Executable {
    pub(crate) fn new(
        id: usize,
        name: String,
        sources: Vec<String>,
        include_dirs: Vec<String>,
    ) -> Self {
        Self {
            id,
            name,
            sources,
            include_dirs,
        }
    }

    pub(crate) fn make_ref(&self) -> ExeRef {
        ExeRef::new(self.id)
    }

    pub(crate) fn get_name(&self) -> &String {
        &self.name
    }
    pub(crate) fn get_sources(&self) -> &Vec<String> {
        &self.sources
    }
    pub(crate) fn get_include_dirs(&self) -> &Vec<String> {
        &self.include_dirs
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
    base_location: Location,
    name: &str,
    name_location: TokLoc,
    frame: &EnvFrame,
) -> Value<Box<dyn ValueTypeMarker>> {
    match name {
        _ => {
            push_diagnostic(
                UnknownPropertyError::new(
                    ExprLocAndType::new(base_location, TypeId::ExecutableReference.typename()),
                    name,
                    name_location.as_rng(),
                ),
                frame,
            );
            Value::new(Box::new(ErrorValue::new()))
        }
    }
}
