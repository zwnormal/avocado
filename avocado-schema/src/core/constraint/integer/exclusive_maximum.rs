use crate::base::{SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExclusiveMaximum {
    #[serde(rename = "exclusiveMaximum")]
    pub max_val: i64,
}

impl Constraint for ExclusiveMaximum {
    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::Number(v) if v.is_i64() && (v.as_i64().unwrap() >= self.max_val) => {
                Err(SchemaError::Validation {
                    message: format!("The {} is larger then or equals to {}", v, self.max_val),
                    constraint_name: "ExclusiveMaximum".to_string(),
                })
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::integer::exclusive_maximum::ExclusiveMaximum;
    use crate::core::constraint::Constraint;
    use serde_json::Value;

    #[test]
    fn test_exclusive_maximum() {
        let constraint = ExclusiveMaximum { max_val: 10 };

        let value = Value::Number(3.into());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(10.into());
        assert!(constraint.validate(&value).is_err());

        let value = Value::Number(11.into());
        assert!(constraint.validate(&value).is_err());
    }
}
