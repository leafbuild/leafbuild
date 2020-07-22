use crate::interpreter::{
    get_global_functions, FuncCallExecutor, FuncCallPool, Value, ValueTypeMarker,
};

use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(PartialOrd, PartialEq, Debug)]
pub(crate) enum TypeIdAndValue<'a> {
    I32(&'a i32),
    I64(&'a i64),
    U32(&'a u32),
    U64(&'a u64),
    String(&'a String),
}

impl<'a> Display for TypeIdAndValue<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub(crate) enum TypeId {
    I32,
    I64,
    U32,
    U64,
    String,
}

impl Display for TypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

include!("int.rs");
include!("string.rs");
