use anyhow::Result;
use serde_json::Value;

pub trait Constraint {
    fn validate(&self, val: &Value) -> Result<()>;
}

pub mod common;
pub mod float;
pub mod integer;
pub mod object;
pub mod string;
