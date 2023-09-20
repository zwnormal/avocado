use crate::base::{SchemaResult, Value};
use crate::core::constraint::Constraint;
use regex::Regex;

#[derive(Debug)]
pub struct Pattern {
    pub pattern: Regex,
}

impl Constraint for Pattern {
    fn verify(&self) -> SchemaResult {
        Ok(())
    }

    fn validate(&self, val: &Value) -> SchemaResult {
        todo!()
    }
}
