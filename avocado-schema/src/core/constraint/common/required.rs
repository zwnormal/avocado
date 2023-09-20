use crate::base::{SchemaError, SchemaResult, Value};
use crate::core::constraint::Constraint;

#[derive(Debug)]
pub struct Required {
    pub required: bool,
}

impl Constraint for Required {
    fn verify(&self) -> SchemaResult {
        Ok(())
    }

    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::Null => {
                if self.required {
                    Err(SchemaError::VerificationFailed {
                        message: "The value is required".to_string(),
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

#[cfg(test)]
mod tests {
    use crate::base::Value;
    use crate::core::constraint::common::required::Required;
    use crate::core::constraint::Constraint;

    #[test]
    fn validate_required() {
        let constraint = Required { required: true };

        let value = Value::String("Test".to_string());
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Null;
        assert!(constraint.validate(&value).is_err())
    }
}
