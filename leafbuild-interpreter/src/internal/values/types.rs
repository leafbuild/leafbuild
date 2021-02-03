use crate::internal::values::BuiltinTy;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Ty {
    I32,
    I64,
    U32,
    U64,
    Bool,
    Str,

    Tuple(Vec<Ty>),

    Fn(FnTy),

    BuiltinObject(BuiltinTy),
    Object(ObjectTy),

    Unknown,
}

pub use Ty::Unknown as TyUnknown;

impl Default for Ty {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FnTy {
    pub positional_params: Vec<FnPositionalTy>,
    pub default_params: Vec<FnDefaultTy>,

    pub ret_ty: Box<Ty>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FnPositionalTy {
    pub name: String,
    pub ty: Ty,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FnDefaultTy {
    name: String,
    ty: Ty,
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
            Self::Str => write!(f, "str"),
            Self::Tuple(tuple_values) => write!(
                f,
                "({})",
                tuple_values.iter().map(ToString::to_string).join(", ")
            ),
            Self::Fn(FnTy {
                positional_params,
                default_params,
                ret_ty,
            }) => {
                write!(
                    f,
                    "fn({}) -> {}",
                    positional_params
                        .iter()
                        .map(|it| format!("{}: {}", &it.name, &it.ty))
                        .chain(
                            default_params
                                .iter()
                                .map(|it| format!("{}: {} = (default)", &it.name, &it.ty))
                        )
                        .join(", "),
                    ret_ty
                )
            }
            Self::BuiltinObject(builtin_ty) => write!(f, "{}", builtin_ty),
            Self::Object(object_type) => write!(f, "{}", object_type),
            Self::Unknown => write!(f, "<unknown>"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TyProperty {
    pub name: String,
    pub ty: Ty,
    pub is_mut: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TyMethod {
    pub name: String,
    pub params: Ty,
    pub ret_ty: Ty,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TyBlueprint {
    pub properties: Vec<TyProperty>,
    pub methods: Vec<TyMethod>,
    pub indexable_by: Vec<Ty>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
