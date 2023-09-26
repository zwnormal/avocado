use crate::base::{SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Enumeration {
    #[serde(rename = "enum")]
    pub values: Vec<String>,
}

impl Constraint for Enumeration {
    fn verify(&self) -> SchemaResult {
        if self.values.is_empty() {
            Err(SchemaError::Verify {
                message: "Can not be empty".to_string(),
                constraint_name: "Enum of String".to_string(),
            })
        } else {
            Ok(())
        }
    }

    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::String(v) if !self.values.contains(v) => Err(SchemaError::Verification {
                message: format!("The string {} is not valid value", v),
                constraint_name: "Enum of String".to_string(),
            }),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::string::enumeration::Enumeration;
    use crate::core::constraint::Constraint;
    use serde_json::Value;

    #[test]
    fn test_enumeration() {
        let constraint = Enumeration {
            values: vec!["China".to_string(), "Australia".to_string()],
        };

        let value = Value::String("China".to_string());
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("United States".to_string());
        assert!(constraint.validate(&value).is_err());

        let constraint = Enumeration { values: vec![] };
        assert!(constraint.verify().is_err());
    }
}
