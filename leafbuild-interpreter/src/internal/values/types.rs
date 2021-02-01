use crate::internal::values::BuiltinTy;
use std::fmt;

#[derive(Debug)]
pub enum Ty {
    I32,
    I64,
    U32,
    U64,
    Bool,

    Tuple(Vec<Ty>),

    BuiltinObject(BuiltinTy),
    Object(ObjectTy),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use itertools::Itertools as _;
        match self {
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::U32 => write!(f, "u32"),
            Self::U64 => write!(f, "u64"),
            Self::Bool => write!(f, "bool"),
            Self::Tuple(tuple_values) => write!(
                f,
                "({})",
                tuple_values.iter().map(ToString::to_string).join(", ")
            ),
            Self::BuiltinObject(builtin_ty) => write!(f, "{}", builtin_ty),
            Self::Object(object_type) => write!(f, "{}", object_type),
        }
    }
}

#[derive(Debug)]
pub struct TyProperty {
    pub name: String,
    pub ty: Ty,
    pub is_mut: bool,
}

#[derive(Debug)]
pub struct TyMethod {
    pub name: String,
    pub params: Ty,
    pub ret_ty: Ty,
}

#[derive(Debug, Default)]
pub struct TyBlueprint {
    pub properties: Vec<TyProperty>,
    pub methods: Vec<TyMethod>,
    pub indexable_by: Vec<Ty>,
}

#[derive(Debug)]
pub struct ObjectTy {
    pub name: String,
    pub ty_blueprint: TyBlueprint,
}

impl ObjectTy {
    pub const fn new(name: String, ty_blueprint: TyBlueprint) -> Self {
        Self { name, ty_blueprint }
    }
}

impl fmt::Display for ObjectTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}
