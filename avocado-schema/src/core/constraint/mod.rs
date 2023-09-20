use crate::base::Value;

pub trait Constraint {
    fn verify(&self) -> Result<(), String>;
    fn validate(&self, val: &Value) -> Result<(), String>;
}

pub mod common;
pub mod integer;
pub mod string;
