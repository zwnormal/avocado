use crate::base::{number_as_i64, SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use serde_json::Value;

#[derive(Debug)]
pub struct Minimum {
    pub min_val: i64,
}

impl Constraint for Minimum {
    fn verify(&self) -> SchemaResult {
        Ok(())
    }

    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::Number(v) if number_as_i64(v)? < self.min_val => {
                Err(SchemaError::VerificationFailed {
                    message: format!("The {} is less then {}", v, self.min_val),
                    constraint_name: "Minimum".to_string(),
                })
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::integer::minimum::Minimum;
    use crate::core::constraint::Constraint;
    use serde_json::Value;

    #[test]
    fn test_minimum() {
        let constraint = Minimum { min_val: 10 };

        let value = Value::Number(11.into());
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(10.into());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(3.into());
        assert!(constraint.validate(&value).is_err());
    }
}
