use crate::base::{number_as_i64, SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Enumeration {
    #[serde(rename = "enum")]
    pub values: Vec<i64>,
}

impl Constraint for Enumeration {
    fn verify(&self) -> SchemaResult {
        if self.values.is_empty() {
            Err(SchemaError::Verify {
                message: "Can not be empty".to_string(),
                constraint_name: "Enum of Integer".to_string(),
            })
        } else {
            Ok(())
        }
    }

    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::Number(v) if !self.values.contains(&number_as_i64(v)?) => {
                Err(SchemaError::Verification {
                    message: format!("The integer {} is not valid value", v),
                    constraint_name: "Enum of Integer".to_string(),
                })
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::integer::enumeration::Enumeration;
    use crate::core::constraint::Constraint;
    use serde_json::Value;

    #[test]
    fn test_enumeration() {
        let constraint = Enumeration { values: vec![1, 2] };

        let value = Value::Number(1.into());
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(3.into());
        assert!(constraint.validate(&value).is_err());

        let constraint = Enumeration { values: vec![] };
        assert!(constraint.verify().is_err());
    }
}
