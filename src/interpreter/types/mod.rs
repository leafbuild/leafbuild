use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::path::PathBuf;

use itertools::Itertools;

use libutils::compilers::flags::{CompilationFlag, CompilationFlags, Flag, LinkFlag, LinkFlags};
use libutils::utils::{Language, NotALanguageError};

use crate::grammar::ast::{AstLoc, Expr};
use crate::grammar::TokLoc;
use crate::interpreter::diagnostics::errors::{
    ExpectedTypeError, ExprLocAndType, InvalidTargetPropertyError, UnexpectedTypeInArray,
    UnknownPropertyError,
};
use crate::interpreter::diagnostics::{push_diagnostic, DiagnosticsCtx, Location};
use crate::interpreter::{
    diagnostics, CallExecutor, CallPool, Env, EnvFrame, LeafTask, Value, ValueTypeMarker,
};

pub(crate) enum TypeIdAndValue<'a> {
    I32(&'a i32),
    I64(&'a i64),
    U32(&'a u32),
    U64(&'a u64),
    Bool(&'a bool),
    String(&'a String),
    Void,
    Error,
    Vec(&'a Vec<Value<Box<dyn ValueTypeMarker>>>),
    Map(&'a HashMap<String, Value<Box<dyn ValueTypeMarker>>>),
    ExecutableReference(&'a ExeRef),
    LibraryReference(&'a LibRef),
    MapPair(&'a MapPair),
    LibType(LibType),
    TargetProperties(&'a TargetProperties),
    OnOff(&'a OnOff),
}

impl<'a> TypeIdAndValue<'a> {
    pub(crate) fn cast_to_usize(&self) -> Result<usize, TypeId> {
        match self {
            TypeIdAndValue::I32(&v) => Ok(v as usize),
            TypeIdAndValue::I64(&v) => Ok(v as usize),
            TypeIdAndValue::U32(&v) => Ok(v as usize),
            TypeIdAndValue::U64(&v) => Ok(v as usize),
            v => Err(v.degrade()),
        }
    }

    pub(crate) fn get_string(&'a self) -> Result<&'a String, TypeId> {
        match self {
            TypeIdAndValue::String(s) => Ok(s),
            v => Err(v.degrade()),
        }
    }

    pub(crate) fn get_bool(&'a self) -> Result<bool, TypeId> {
        match self {
            TypeIdAndValue::Bool(&v) => Ok(v),
            v => Err(v.degrade()),
        }
    }

    pub(crate) fn get_map(
        &'a self,
    ) -> Result<&'a HashMap<String, Value<Box<dyn ValueTypeMarker>>>, TypeId> {
        match self {
            TypeIdAndValue::Map(m) => Ok(*m),
            v => Err(v.degrade()),
        }
    }

    pub(crate) fn get_map_pair(&'a self) -> Result<&'a MapPair, TypeId> {
        match self {
            TypeIdAndValue::MapPair(v) => Ok(*v),
            v => Err(v.degrade()),
        }
    }

    pub(crate) fn get_lib_type(&self) -> Result<LibType, TypeId> {
        match self {
            TypeIdAndValue::LibType(v) => Ok(*v),
            v => Err(v.degrade()),
        }
    }

    pub(crate) fn get_on_off(&self) -> Result<OnOff, TypeId> {
        match self {
            TypeIdAndValue::OnOff(on_off) => Ok(**on_off),
            tp => Err(tp.degrade()),
        }
    }

    /// Given a string or a vector of strings in a TypeIdAndValue, this function returns the Some
    /// option of the vector of strings if the TypeIdAndValue is a vector, or just a vector with only
    /// the string if it is alone, and pushes any diagnostics straight to the diagnostics context.
    ///
    /// It should return None if `self` is not either a string or a vector.
    pub(crate) fn to_vec_of_string(
        &self,
        location: Location,
        docs: Option<&str>,
        diag_ctx: &DiagnosticsCtx,
    ) -> Option<Vec<String>> {
        match self {
            TypeIdAndValue::String(v) => Some(vec![(*v).clone()]),
            TypeIdAndValue::Vec(vec) => Some(
                vec.iter()
                    .enumerate()
                    .filter_map(
                        |(idx, v)| match v.get_type_id_and_value_required(TypeId::String) {
                            Ok(s) => Some(s.get_string().unwrap().clone()),
                            Err(tp) => {
                                diagnostics::push_diagnostic_ctx(
                                    UnexpectedTypeInArray::new(
                                        location.clone(),
                                        tp.typename(),
                                        TypeId::String.typename(),
                                        idx,
                                    )
                                    .with_docs_location_opt(docs.map(|v| v.to_string())),
                                    diag_ctx,
                                );
                                None
                            }
                        },
                    )
                    .collect_vec(),
            ),
            tp => {
                diagnostics::push_diagnostic_ctx(
                    ExpectedTypeError::new(
                        TypeId::Vec.typename(),
                        ExprLocAndType::new(location, tp.degrade().typename()),
                    )
                    .with_docs_location_opt(docs.map(|v| v.to_string())),
                    diag_ctx,
                );
                None
            }
        }
    }

    pub(crate) fn to_vec_of_dependencies(
        &self,
        location: Location,
        docs: Option<&str>,
        diag_ctx: &DiagnosticsCtx,
    ) -> Option<Vec<Box<dyn Dependency>>> {
        match self {
            TypeIdAndValue::Vec(vec) => Some(
                vec.iter()
                    .enumerate()
                    .filter_map(|(idx, v)| -> Option<Box<dyn Dependency>> {
                        match v.get_type_id_and_value() {
                            TypeIdAndValue::LibraryReference(ref_) => Some(Box::new(*ref_)),
                            tp => {
                                diagnostics::push_diagnostic_ctx(
                                    UnexpectedTypeInArray::new(
                                        location.clone(),
                                        tp.degrade().typename(),
                                        TypeId::LibraryReference.typename(),
                                        idx,
                                    )
                                    .with_docs_location_opt(docs.map(|v| v.to_string())),
                                    diag_ctx,
                                );
                                None
                            }
                        }
                    })
                    .collect_vec(),
            ),
            tp => {
                diagnostics::push_diagnostic_ctx(
                    ExpectedTypeError::new(
                        TypeId::Vec.typename(),
                        ExprLocAndType::new(location, tp.degrade().typename()),
                    )
                    .with_docs_location_opt(match docs {
                        Some(v) => Some(v.to_string()),
                        None => None,
                    }),
                    diag_ctx,
                );
                None
            }
        }
    }
}

impl<'a> TypeIdAndValue<'a> {
    pub(crate) fn stringify(&self) -> String {
        match self {
            TypeIdAndValue::I32(&v) => format!("{}", v),
            TypeIdAndValue::I64(&v) => format!("{}", v),
            TypeIdAndValue::U32(&v) => format!("{}", v),
            TypeIdAndValue::U64(&v) => format!("{}", v),
            TypeIdAndValue::Bool(&v) => format!("{}", v),
            TypeIdAndValue::String(v) => (*v).clone(),
            TypeIdAndValue::Void => "".to_string(),
            TypeIdAndValue::Error => "(error)".to_string(),
            TypeIdAndValue::Vec(v) => v.stringify(),
            TypeIdAndValue::Map(v) => v.stringify(),
            TypeIdAndValue::ExecutableReference(v) => v.stringify(),
            TypeIdAndValue::LibraryReference(v) => v.stringify(),
            TypeIdAndValue::MapPair(v) => v.stringify(),
            TypeIdAndValue::LibType(v) => v.stringify(),
            TypeIdAndValue::TargetProperties(v) => v.stringify(),
            TypeIdAndValue::OnOff(v) => v.stringify(),
        }
    }

    #[inline]
    pub(crate) fn degrade(&self) -> TypeId {
        match self {
            TypeIdAndValue::I32(_) => TypeId::I32,
            TypeIdAndValue::I64(_) => TypeId::I64,
            TypeIdAndValue::U32(_) => TypeId::U32,
            TypeIdAndValue::U64(_) => TypeId::U64,
            TypeIdAndValue::Bool(_) => TypeId::Bool,
            TypeIdAndValue::String(_) => TypeId::String,
            TypeIdAndValue::Void => TypeId::Void,
            TypeIdAndValue::Error => TypeId::Error,
            TypeIdAndValue::Vec(_) => TypeId::Vec,
            TypeIdAndValue::Map(_) => TypeId::Map,
            TypeIdAndValue::ExecutableReference(_) => TypeId::ExecutableReference,
            TypeIdAndValue::MapPair(_) => TypeId::MapPair,
            TypeIdAndValue::LibraryReference(_) => TypeId::LibraryReference,
            TypeIdAndValue::LibType(_) => TypeId::LibType,
            TypeIdAndValue::TargetProperties(_) => TypeId::TargetProperties,
            TypeIdAndValue::OnOff(_) => TypeId::OnOff,
        }
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub(crate) enum TypeId {
    I32,
    I64,
    U32,
    U64,
    Bool,
    String,
    Void,
    Error,
    Vec,
    Map,
    ExecutableReference,
    LibraryReference,
    MapPair,
    LibType,
    TargetProperties,
    OnOff,
}

impl TypeId {
    #[inline]
    pub(crate) fn typename(&self) -> &'static str {
        match self {
            TypeId::I32 => "i32",
            TypeId::I64 => "i64",
            TypeId::U32 => "u32",
            TypeId::U64 => "u64",
            TypeId::Bool => "bool",
            TypeId::String => "string",
            TypeId::Void => "void",
            TypeId::Error => "error",
            TypeId::Vec => "vec",
            TypeId::Map => "map",
            TypeId::ExecutableReference => "exe_ref",
            TypeId::LibraryReference => "lib_ref",
            TypeId::MapPair => "map_pair",
            TypeId::LibType => "lib_type",
            TypeId::TargetProperties => "target_properties",
            TypeId::OnOff => "on_off",
        }
    }
}

impl Display for TypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.typename())
    }
}

include!("primitives/int.rs");
include!("primitives/bool.rs");
include!("primitives/string.rs");
include!("primitives/void.rs");
include!("primitives/error.rs");

include!("primitives/libtype.rs");

include!("dependency.rs");

include!("vec.rs");
include!("map.rs");
include!("map_pair.rs");
include!("executable.rs");
include!("library.rs");
include!("target_properties.rs");
