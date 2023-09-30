use crate::base::SchemaResult;
use serde_json::Value;

pub trait Constraint {
    fn validate(&self, val: &Value) -> SchemaResult;
}

pub mod common;
pub mod float;
pub mod integer;
pub mod object;
pub mod string;
