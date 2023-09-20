use crate::base::{SchemaError, SchemaResult, Value};
use crate::core::constraint::Constraint;

#[derive(Debug)]
pub struct Required {
    pub required: Vec<String>,
}

impl Constraint for Required {
    fn verify(&self) -> SchemaResult {
        Ok(())
    }

    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::Object(o) => {
                let mut missing_fields = vec![];
                for field in &self.required {
                    match o.get(field.as_str()) {
                        None => {
                            missing_fields.push(field.clone());
                        }
                        Some(v) => match v {
                            Value::Null => {
                                missing_fields.push(field.clone());
                            }
                            _ => {}
                        },
                    }
                }

                if missing_fields.len() > 0 {
                    Err(SchemaError::VerificationFailed {
                        message: format!("[{}] field(s) are required", missing_fields.join(", ")),
                        constraint_name: "Required".to_string(),
                    })
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}
