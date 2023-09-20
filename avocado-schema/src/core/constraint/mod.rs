use crate::base::{SchemaResult, Value};

pub trait Constraint {
    fn verify(&self) -> SchemaResult;
    fn validate(&self, val: &Value) -> SchemaResult;
}

pub mod common;
pub mod integer;
pub mod object;
pub mod string;
