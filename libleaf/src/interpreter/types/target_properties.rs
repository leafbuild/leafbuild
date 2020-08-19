#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum OnOff {
    Off = 0,
    On = 1,
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
pub(crate) struct TargetProperties {
    pic: OnOff,
    language: Language,
}

impl ValueTypeMarker for TargetProperties {
    fn stringify(&self) -> String {
        format!(
            "target_properties: {{ pic = {}, language = {:?} }}",
            self.pic, self.language
        )
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        Value::new(Box::new(*self))
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::TargetProperties
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue {
        TypeIdAndValue::TargetProperties(self)
    }
}

pub(crate) struct InvalidTargetProperty {
    msg: String,
    note: Option<String>,
}

impl InvalidTargetProperty {
    pub(crate) fn new(msg: impl Into<String>, note: Option<&str>) -> Self {
        Self {
            msg: msg.into(),
            note: match note {
                Some(v) => Some(v.into()),
                None => None,
            },
        }
    }

    pub(crate) fn get_message(&self) -> &String {
        &self.msg
    }
    pub(crate) fn get_note(&self) -> &Option<String> {
        &self.note
    }
}

impl From<NotALanguageError> for InvalidTargetProperty {
    fn from(err: NotALanguageError) -> Self {
        InvalidTargetProperty::new(err.get_msg(), None)
    }
}

impl TryFrom<&Value<Box<dyn ValueTypeMarker>>> for OnOff {
    type Error = InvalidTargetProperty;

    fn try_from(value: &Value<Box<dyn ValueTypeMarker>>) -> Result<Self, Self::Error> {
        match value.get_type_id_and_value().get_on_off() {
            Ok(v) => Ok(v),
            Err(tp) => Err(InvalidTargetProperty::new(
                format!(
                    "Cannot parse OnOff from {} (of type {})",
                    value.stringify(),
                    tp.typename(),
                ),
                match tp {
                    TypeId::Bool => {
                        Some("Consider converting this bool to OnOff with `( ... ? on : off )'")
                    }
                    _ => None,
                },
            )),
        }
    }
}

impl TargetProperties {
    pub(crate) fn from_expr(value: &Expr, frame: &mut EnvFrame) -> Self {
        let mut pic: OnOff = OnOff::Off;
        let mut language: Language = Language::C;
        match value
            .eval_in_env(frame)
            .get_type_id_and_value_required(TypeId::Map)
        {
            Ok(map) => {
                map.get_map()
                    .unwrap()
                    .into_iter()
                    .for_each(|val| match val.0 as &str {
                        "position_independent_code" | "pic" => match OnOff::try_from(val.1) {
                            Ok(v) => {
                                pic = v;
                            }
                            Err(err) => {
                                diagnostics::push_diagnostic(
                                    InvalidTargetPropertyError::new(
                                        err,
                                        value.get_rng(),
                                        val.0,
                                        val.1,
                                    ),
                                    frame,
                                );
                            }
                        },
                        "language" => match val.1.get_type_id_and_value_required(TypeId::String) {
                            Ok(v) => match v.get_string().unwrap().parse() {
                                Ok(lng) => {
                                    language = lng;
                                }
                                Err(err) => {
                                    diagnostics::push_diagnostic(
                                        InvalidTargetPropertyError::new(
                                            InvalidTargetProperty::from(err),
                                            value.get_rng(),
                                            val.0,
                                            val.1,
                                        ),
                                        frame,
                                    );
                                }
                            },
                            Err(tp) => {
                                diagnostics::push_diagnostic(
                                    InvalidTargetPropertyError::new(
                                        InvalidTargetProperty::new(
                                            format!(
                                                "Cannot parse language from non-string '{}'",
                                                val.1.stringify()
                                            ),
                                            None,
                                        ),
                                        value.get_rng(),
                                        val.0,
                                        val.1,
                                    ),
                                    frame,
                                );
                            }
                        },
                        v => {
                            diagnostics::push_diagnostic(
                                InvalidTargetPropertyError::new(
                                    InvalidTargetProperty::new(
                                        format!("Unknown target property: {}", v),
                                        None,
                                    ),
                                    value.get_rng(),
                                    val.0,
                                    val.1,
                                ),
                                frame,
                            );
                        }
                    });
            }
            Err(tp) => diagnostics::push_diagnostic(
                ExpectedTypeError::new(
                    TypeId::Map.typename(),
                    ExprLocAndType::new(value.get_rng(), tp.typename()),
                ),
                frame,
            ),
        }
        TargetProperties { pic, language }
    }
}

impl Default for TargetProperties {
    fn default() -> Self {
        Self {
            pic: OnOff::Off,
            language: Language::C,
        }
    }
}

pub(crate) fn get_target_properties_call_pool() -> CallPool {
    CallPool::new(vec![])
}

pub(crate) fn get_on_off_call_pool() -> CallPool {
    CallPool::new(vec![])
}

pub(crate) fn resolve_target_properties_type_property_access(
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
                    ExprLocAndType::new(base_location, TypeId::TargetProperties.typename()),
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
