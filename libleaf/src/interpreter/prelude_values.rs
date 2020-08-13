use std::collections::HashMap;

use crate::interpreter::types::LibType;
use crate::interpreter::{Value, ValueTypeMarker};
use std::iter::FromIterator;

pub(crate) fn get_prelude_values() -> HashMap<String, Value<Box<dyn ValueTypeMarker>>> {
    let v: Vec<(String, Value<Box<dyn ValueTypeMarker>>)> = vec![
        ("static".to_string(), Value::new(Box::new(LibType::Static))),
        ("shared".to_string(), Value::new(Box::new(LibType::Shared))),
    ];
    HashMap::from_iter(v.into_iter())
}
