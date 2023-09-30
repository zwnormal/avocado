use crate::base::field::number_as_f64;
use crate::base::{SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Enumeration {
    #[serde(rename = "enum")]
    pub values: Vec<f64>,
}

impl Constraint for Enumeration {
    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::Number(v) if !self.values.contains(&number_as_f64(v)?) => {
                Err(SchemaError::Verification {
                    message: format!("The float {} is not valid value", v),
                    constraint_name: "Enum of Float".to_string(),
                })
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::float::enumeration::Enumeration;
    use crate::core::constraint::Constraint;
    use serde_json::{Number, Value};

    #[test]
    fn test_enumeration() {
        let constraint = Enumeration {
            values: vec![1.1, 2.5],
        };

        let value = Value::Number(Number::from_f64(1.1).unwrap());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(Number::from_f64(3.0).unwrap());
        assert!(constraint.validate(&value).is_err());
    }
}
