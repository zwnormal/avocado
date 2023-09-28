use crate::base::field::number_as_f64;
use crate::base::{SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExclusiveMinimum {
    #[serde(rename = "exclusiveMinimum")]
    pub min_val: f64,
}

impl Constraint for ExclusiveMinimum {
    fn verify(&self) -> SchemaResult {
        Ok(())
    }

    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::Number(v) if number_as_f64(v)? <= self.min_val => {
                Err(SchemaError::Verification {
                    message: format!("The {} is less then or equals to {}", v, self.min_val),
                    constraint_name: "ExclusiveMinimum".to_string(),
                })
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::float::exclusive_minimum::ExclusiveMinimum;
    use crate::core::constraint::Constraint;
    use serde_json::{Number, Value};

    #[test]
    fn test_exclusive_minimum() {
        let constraint = ExclusiveMinimum { min_val: 10.0 };

        let value = Value::Number(Number::from_f64(11.0).unwrap());
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(Number::from_f64(10.0).unwrap());
        assert!(constraint.validate(&value).is_err());

        let value = Value::Number(Number::from_f64(3.0).unwrap());
        assert!(constraint.validate(&value).is_err());
    }
}
