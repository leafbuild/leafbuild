#[derive(Clone)]
pub(crate) struct Library {
    name: String,
    type_: LibType,
    mod_id: usize,
    id: usize,
    sources: Vec<String>,
    internal_include_dirs: Vec<String>,
    external_include_dirs: Vec<String>,
    properties: TargetProperties,
}

impl Library {
    pub(crate) fn new(
        id: usize,
        mod_id: usize,
        name: String,
        type_: LibType,
        sources: Vec<String>,
        internal_include_dirs: Vec<String>,
        external_include_dirs: Vec<String>,
        properties: TargetProperties,
    ) -> Self {
        Self {
            id,
            mod_id,
            type_,
            name,
            sources,
            internal_include_dirs,
            external_include_dirs,
            properties,
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
    pub(crate) fn get_sources(&self) -> &[String] {
        &self.sources
    }
    pub(crate) fn get_internal_include_dirs(&self) -> &[String] {
        &self.internal_include_dirs
    }
    pub(crate) fn get_external_include_dirs(&self) -> &[String] {
        &self.external_include_dirs
    }
    // pub(crate) fn get_properties(&self) -> &Vec<TargetProperty> {
    //     &self.properties
    // }

    pub(crate) fn validate(&self) -> Result<(), LibraryValidationError> {
        if let LibType::Shared = self.type_ {
            if self.properties.pic == OnOff::Off {
                return Err(LibraryValidationError::new("Cannot create a shared library with property `position_independent_code = off'"));
            }
        }
        Ok(())
    }
}

pub(crate) struct LibraryValidationError {
    msg: String,
}

impl LibraryValidationError {
    pub(crate) fn new(msg: impl Into<String>) -> Self {
        Self { msg: msg.into() }
    }

    pub(crate) fn get_message(self) -> String {
        self.msg
    }
}

#[derive(Copy, Clone)]
pub(crate) struct LibRef {
    id: usize,
}

impl LibRef {
    pub(crate) fn new(id: usize) -> Self {
        Self { id }
    }

    // fn get_lib_in_env<'a>(&self, env: &'a Env) -> &'a Library {
    //     env.mut_
    //         .libraries
    //         .iter()
    //         .find(|lib| lib.id == self.id)
    //         .unwrap()
    // }
}

impl ValueTypeMarker for LibRef {
    fn stringify(&self) -> String {
        format!("library reference {{ id = '{}' }}", self.id)
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(LibRef { id: self.id }))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::LibraryReference
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue {
        TypeIdAndValue::LibraryReference(self)
    }
}

impl Dependency for LibRef {
    fn get_implicit_requirements(&self, language: Language, env: &Env) -> Vec<String> {
        // let lib = self.get_lib_in_env(env);
        // vec![lib.type_.fmt_name(&lib.name)]
        vec![]
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
