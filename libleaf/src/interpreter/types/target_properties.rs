#[derive(Copy, Clone)]
#[repr(u8)]
pub(crate) enum OnOff {
    On,
    Off,
}

impl Display for OnOff {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OnOff::On => "on",
                OnOff::Off => "off",
            }
        )
    }
}

impl ValueTypeMarker for OnOff {
    fn stringify(&self) -> String {
        format!("{}", self)
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(*self))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::OnOff
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue {
        TypeIdAndValue::OnOff(self)
    }
}

#[derive(Copy, Clone)]
pub(crate) enum TargetProperty {
    PositionIndependentCode(OnOff),
    Language(Language),
}

impl ValueTypeMarker for TargetProperty {
    fn stringify(&self) -> String {
        match self {
            TargetProperty::PositionIndependentCode(on_off) => {
                format!("target_property {{ pic = {} }}", on_off)
            }
            TargetProperty::Language(lang) => {
                format!("target_property {{ language = {:?} }}", lang)
            }
        }
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(*self))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::TargetProperty
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue {
        TypeIdAndValue::TargetProperty(self)
    }
}

pub(crate) struct InvalidTargetProperty {
    msg: String,
}

impl InvalidTargetProperty {
    pub(crate) fn new(msg: impl Into<String>) -> Self {
        Self { msg: msg.into() }
    }

    pub(crate) fn get_message(&self) -> &String {
        &self.msg
    }
}

impl From<NotALanguageError> for InvalidTargetProperty {
    fn from(err: NotALanguageError) -> Self {
        InvalidTargetProperty::new(err.get_msg())
    }
}

impl TryFrom<&Value<Box<dyn ValueTypeMarker>>> for OnOff {
    type Error = InvalidTargetProperty;

    fn try_from(value: &Value<Box<dyn ValueTypeMarker>>) -> Result<Self, Self::Error> {
        match value.get_type_id_and_value().get_on_off() {
            Ok(v) => Ok(v),
            Err(_) => Err(InvalidTargetProperty::new(format!(
                "Cannot parse OnOff from {}",
                value.stringify()
            ))),
        }
    }
}

impl TryFrom<(&String, &Value<Box<dyn ValueTypeMarker>>)> for TargetProperty {
    type Error = InvalidTargetProperty;

    fn try_from(value: (&String, &Value<Box<dyn ValueTypeMarker>>)) -> Result<Self, Self::Error> {
        match value.0 as &str {
            "position_independent_code" => {
                Ok(Self::PositionIndependentCode(OnOff::try_from(value.1)?))
            }
            "language" => match value.1.get_type_id_and_value_required(TypeId::String) {
                Ok(v) => Ok(Self::Language({
                    let lang: Language = v.get_string().unwrap().parse()?;
                    lang
                })),
                Err(tp) => Err(InvalidTargetProperty::new(format!(
                    "Cannot parse language from non-string '{}'",
                    value.1.stringify()
                ))),
            },
            v => Err(InvalidTargetProperty::new(format!(
                "Unknown target property: {}",
                v
            ))),
        }
    }
}

pub(crate) fn get_target_property_call_pool() -> CallPool {
    CallPool::new(vec![])
}

pub(crate) fn get_on_off_call_pool() -> CallPool {
    CallPool::new(vec![])
}

pub(crate) fn resolve_target_property_type_property_access(
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
                    ExprLocAndType::new(base_location, TypeId::TargetProperty.typename()),
                    name,
                    name_location.as_rng(),
                ),
                frame,
            );
            Value::new(Box::new(ErrorValue::new()))
        }
    }
}

pub(crate) fn resolve_on_off_type_property_access(
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
                    ExprLocAndType::new(base_location, TypeId::OnOff.typename()),
                    name,
                    name_location.as_rng(),
                ),
                frame,
            );
            Value::new(Box::new(ErrorValue::new()))
        }
    }
}
