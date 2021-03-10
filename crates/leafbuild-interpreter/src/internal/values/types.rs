use std::fmt;

#[derive(Debug)]
pub enum ValueType {
    I32,
    I64,
    U32,
    U64,
    Bool,

    String,

    Vector(Box<ValueType>),
    Map(Box<ValueType>, Box<ValueType>),

    Tuple(Vec<ValueType>),

    Object(ObjectType),
}

#[derive(Debug)]
pub struct ObjectType {
    name: String,
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use itertools::Itertools as _;
        match self {
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::U32 => write!(f, "u32"),
            Self::U64 => write!(f, "u64"),
            Self::Bool => write!(f, "bool"),
            Self::String => write!(f, "string"),
            Self::Vector(v) => write!(f, "vector<{v}>", v = v),
            Self::Map(k, v) => write!(f, "map<{k}, {v}>", k = k, v = v),
            Self::Tuple(tuple_values) => write!(
                f,
                "({})",
                tuple_values.iter().map(ToString::to_string).join(", ")
            ),
            Self::Object(object_type) => write!(f, "{}", object_type),
        }
    }
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "object(name={name})", name = &self.name)
    }
}
