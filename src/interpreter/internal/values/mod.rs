pub mod types;

use crate::grammar::Span;
use thiserror::Error;
use types::ValueType;

#[derive(Error, Debug)]
pub enum GetPropertyError {
    #[error("no such property `{name}' on a value of type {root_type:?}")]
    NoSuchProperty {
        root_type: ValueType,
        root_span: Span,
        dot_span: Span,
        name: String,
        name_span: Span,
    },
}

#[derive(Error, Debug)]
pub enum InvokeMethodError {
    #[error("no such method `{name}' on a value of type {root_type:?}")]
    NoSuchMethod {
        root_type: ValueType,
        root_span: Span,
        dot_span: Span,
        name: String,
        name_span: Span,
    },
}

pub trait Value<'a> {
    fn get_property(
        &self,
        this_span: Span,
        dot_span: Span,
        property_name: Span,
        property_name_span: Span,
    ) -> Result<&'a dyn Value<'a>, GetPropertyError>;

    fn invoke_method(
        &self,
        this_span: Span,
        dot_span: Span,
        method_name: &'a str,
        method_name_span: Span,
    ) -> Result<Box<dyn Value<'static>>, InvokeMethodError>;
}
