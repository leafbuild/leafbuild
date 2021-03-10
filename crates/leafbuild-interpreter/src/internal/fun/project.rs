//! Module holding the 'project' function
use crate::env::FileFrame;
use crate::internal::values::{I32Wrap, Value};

pub fn project<'file_frame>(file_frame: &'file_frame mut FileFrame) -> Box<dyn Value<'static>> {
    Box::new(I32Wrap(0))
}
