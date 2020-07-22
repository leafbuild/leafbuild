use crate::grammar::ast::Expr;
use crate::interpreter::{
    get_global_functions, FuncCallExecutor, FuncCallPool, Value, ValueTypeMarker,
};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::{fmt, io};

#[derive(PartialOrd, PartialEq, Debug)]
pub(crate) enum TypeId<'a> {
    I32(&'a i32),
    I64(&'a i64),
    U32(&'a u32),
    U64(&'a u64),
    String(&'a String),
}

impl<'a> Display for TypeId<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

include!("int.rs");
include!("string.rs");
