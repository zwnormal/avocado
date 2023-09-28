use crate::base::{number_as_i64, SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Maximum {
    #[serde(rename = "maximum")]
    pub max_val: i64,
}

impl Constraint for Maximum {
    fn verify(&self) -> SchemaResult {
        Ok(())
    }

    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::Number(v) if number_as_i64(v)? > self.max_val => {
                Err(SchemaError::Verification {
                    message: format!("The {} is larger then {}", v, self.max_val),
                    constraint_name: "Maximum".to_string(),
                })
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::integer::maximum::Maximum;
    use crate::core::constraint::Constraint;
    use serde_json::Value;

    #[test]
    fn test_maximum() {
        let constraint = Maximum { max_val: 10 };

        let value = Value::Number(3.into());
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(10.into());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(11.into());
        assert!(constraint.validate(&value).is_err());
    }
}
