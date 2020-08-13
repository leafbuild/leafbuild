#[derive(Clone)]
pub(crate) struct Library {
    name: String,
    type_: LibType,
    mod_id: usize,
    id: usize,
    sources: Vec<String>,
    internal_include_dirs: Vec<String>,
}

impl Library {
    pub(crate) fn new(
        id: usize,
        mod_id: usize,
        name: String,
        type_: LibType,
        sources: Vec<String>,
        internal_include_dirs: Vec<String>,
    ) -> Self {
        Self {
            id,
            mod_id,
            type_,
            name,
            sources,
            internal_include_dirs,
        }
    }

    pub(crate) fn make_ref(&self) -> LibRef {
        LibRef::new(self.id)
    }

    pub(crate) fn get_mod_id(&self) -> usize {
        self.mod_id
    }
    pub(crate) fn get_name(&self) -> &String {
        &self.name
    }
    pub(crate) fn get_type(&self) -> LibType {
        self.type_
    }
    pub(crate) fn get_sources(&self) -> &Vec<String> {
        &self.sources
    }
    pub(crate) fn get_internal_include_dirs(&self) -> &Vec<String> {
        &self.internal_include_dirs
    }
}

#[derive(Copy, Clone)]
pub(crate) struct LibRef {
    id: usize,
}

impl LibRef {
    fn new(id: usize) -> Self {
        Self { id }
    }
}

impl ValueTypeMarker for LibRef {
    fn stringify(&self) -> String {
        format!("library reference {{ id = '{}' }}", self.id)
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(ExeRef { id: self.id }))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::LibraryReference
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue {
        TypeIdAndValue::LibraryReference(self)
    }
}

impl Dependency for LibRef {
    fn get_cc_compilation_flags(&self) -> &CCFlags {
        unimplemented!()
    }

    fn get_cc_link_flags(&self) -> &CCLDFlags {
        unimplemented!()
    }

    fn get_cxx_compilation_flags(&self) -> &CXXFlags {
        unimplemented!()
    }

    fn get_cxx_link_flags(&self) -> &CXXLDFlags {
        unimplemented!()
    }
}

pub(crate) fn get_library_call_pool() -> CallPool {
    CallPool::new(vec![])
}

pub(crate) fn resolve_library_property_access(
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
                    ExprLocAndType::new(base_location, TypeId::LibraryReference.typename()),
                    name,
                    name_location.as_rng(),
                ),
                frame,
            );
            Value::new(Box::new(ErrorValue::new()))
        }
    }
}
