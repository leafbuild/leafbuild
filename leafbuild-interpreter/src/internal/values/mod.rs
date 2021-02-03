pub mod types;

use crate::env::FileFrame;
use crate::internal::values::types::{TyBlueprint, TyProperty};
use leafbuild_ast::Span;
use once_cell::sync::Lazy;
use std::borrow::Cow;
use std::fmt;
use std::fmt::Debug;
use types::Ty;

pub trait Value<'a>: Debug {
    fn get_type(&self) -> Ty;

    fn get_property(
        &self,
        this_span: Span,
        dot_span: Span,
        property_name: &str,
        property_name_span: Span,
    ) -> ValRef<'a> {
        unreachable!()
    }

    fn get_property_mut(
        &mut self,
        this_span: Span,
        dot_span: Span,
        property_name: &str,
        property_name_span: Span,
    ) -> ValRefMut<'a> {
        unreachable!()
    }

    fn get_indexed(
        &self,
        this_span: Span,
        left_brace: Span,
        index_value: &dyn Value,
        right_brace: Span,
    ) -> ValRef<'a> {
        unreachable!()
    }

    fn get_indexed_mut(
        &mut self,
        this_span: Span,
        left_brace: Span,
        index_value: &dyn Value,
        right_brace: Span,
    ) -> ValRefMut<'a> {
        unreachable!()
    }

    fn invoke_method(
        &self,
        file_frame: &mut FileFrame,
        this_span: Span,
        dot_span: Span,
        method_name: &str,
        method_name_span: Span,
    ) -> Val {
        unreachable!()
    }
}

pub type Val = LiveVal<'static>;
pub type LiveVal<'life> = Box<dyn Value<'life>>;
pub type ValRef<'a> = &'a dyn Value<'a>;
pub type ValRefMut<'a> = &'a mut dyn Value<'a>;

// primitives
include! {"primitives.rs"}
// builtin object types
include! {"builtin.rs"}
