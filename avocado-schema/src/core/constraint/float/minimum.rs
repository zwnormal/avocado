use crate::base::field::number_as_f64;
use crate::base::{SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Minimum {
    #[serde(rename = "minimum")]
    pub min_val: f64,
}

impl Constraint for Minimum {
    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::Number(v) if number_as_f64(v)? < self.min_val => Err(SchemaError::Validation {
                message: format!("The {} is less then {}", v, self.min_val),
                constraint_name: "Minimum".to_string(),
            }),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::float::minimum::Minimum;
    use crate::core::constraint::Constraint;
    use serde_json::{Number, Value};

    #[test]
    fn test_minimum() {
        let constraint = Minimum { min_val: 10.0 };

        let value = Value::Number(Number::from_f64(11.0).unwrap());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(Number::from_f64(10.0).unwrap());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(Number::from_f64(3.0).unwrap());
        assert!(constraint.validate(&value).is_err());
    }
}
