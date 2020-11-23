pub mod types;

use crate::grammar::Span;
use std::fmt::Debug;
use thiserror::Error;
use types::ValueType;

#[derive(Error, Debug)]
pub enum GetPropertyError {
    #[error("no such property `{name}` on a value of type {root_type:?}")]
    NoSuchProperty {
        root_type: ValueType,
        root_span: Span,
        dot_span: Span,
        name: String,
        name_span: Span,
    },
}

#[derive(Error, Debug)]
pub enum GetIndexedError {
    #[error("type cannot be indexed")]
    TypeCannotBeIndexed {
        root_type: ValueType,
        root_span: Span,
        lbrace_span: Span,
        index_type: ValueType,
        rbrace_span: Span,
    },
    #[error("index is of wrong type(`{index_type:?}`) on root type `{root_type:?}`")]
    IndexOfWrongType {
        root_type: ValueType,
        root_span: Span,
        lbrace_span: Span,
        index_type: ValueType,
        rbrace_span: Span,
    },
}

#[derive(Error, Debug)]
pub enum InvokeMethodError {
    #[error("no such method `{name}` on a value of type {root_type:?}")]
    NoSuchMethod {
        root_type: ValueType,
        root_span: Span,
        dot_span: Span,
        name: String,
        name_span: Span,
    },
}

pub trait Value<'a>: Debug {
    fn get_type(&self) -> ValueType;

    fn get_property(
        &self,
        this_span: Span,
        dot_span: Span,
        property_name: &str,
        property_name_span: Span,
    ) -> Result<&'a dyn Value<'a>, GetPropertyError>;

    fn get_property_mut(
        &mut self,
        this_span: Span,
        dot_span: Span,
        property_name: &str,
        property_name_span: Span,
    ) -> Result<&'a mut dyn Value<'a>, GetPropertyError>;

    fn get_indexed(
        &self,
        this_span: Span,
        left_brace: Span,
        index_value: &dyn Value,
        right_brace: Span,
    ) -> Result<&'a dyn Value<'a>, GetIndexedError>;

    fn get_indexed_mut(
        &mut self,
        this_span: Span,
        left_brace: Span,
        index_value: &dyn Value,
        right_brace: Span,
    ) -> Result<&'a mut dyn Value<'a>, GetIndexedError>;

    fn invoke_method(
        &self,
        this_span: Span,
        dot_span: Span,
        method_name: &str,
        method_name_span: Span,
    ) -> Result<Box<dyn Value<'static>>, InvokeMethodError>;
}

include!("i32.rs");
include!("i64.rs");
include!("u32.rs");
include!("u64.rs");
