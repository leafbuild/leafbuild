#[derive(Copy, Clone)]
pub(crate) enum LibType {
    Static,
    Shared,
}

impl LibType {
    #[inline]
    pub(crate) fn fmt_name(&self, name: &str) -> String {
        #[cfg(target_os = "windows")]
        match self {
            LibType::Static => format!("{}.a", name),
            LibType::Shared => format!("{}.dll", name),
        }
        #[cfg(target_os = "linux")]
        match self {
            LibType::Static => format!("lib{}.a", name),
            LibType::Shared => format!("lib{}.so", name),
        }
    }
}

impl ValueTypeMarker for LibType {
    fn stringify(&self) -> String {
        match self {
            LibType::Static => "static",
            LibType::Shared => "shared",
        }
        .to_string()
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(*self))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::LibType
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue {
        TypeIdAndValue::LibType(*self)
    }
}

pub(crate) fn get_lib_type_call_pool() -> CallPool {
    CallPool::new(vec![])
}

pub(crate) fn resolve_lib_type_property_access(
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
                    ExprLocAndType::new(base_location, TypeId::LibType.typename()),
                    name,
                    name_location.as_rng(),
                ),
                frame,
            );
            Value::new(Box::new(ErrorValue::new()))
        }
    }
}
