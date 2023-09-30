use crate::base::{SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MinLength {
    #[serde(rename = "minLength")]
    pub min_length: i64,
}

impl Constraint for MinLength {
    fn verify(&self) -> SchemaResult {
        if self.min_length <= 0 {
            Err(SchemaError::Verify {
                message: "The min length must be larger than 0".to_string(),
                constraint_name: "MinLength".to_string(),
            })
        } else {
            Ok(())
        }
    }

    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::String(v)
                if v.graphemes(true).count()
                    < usize::try_from(self.min_length).map_err(|_| SchemaError::Verify {
                        message: "The min length must be larger than 0".to_string(),
                        constraint_name: "MinLength".to_string(),
                    })? =>
            {
                Err(SchemaError::Verification {
                    message: format!("The length of {} is less then {}", v, self.min_length),
                    constraint_name: "MinLength".to_string(),
                })
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::string::min_length::MinLength;
    use crate::core::constraint::Constraint;
    use serde_json::Value;

    #[test]
    fn test_min_length() {
        let constraint = MinLength { min_length: 8 };

        let value = Value::String("Valid String".to_string());
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("Invalid".to_string());
        assert!(constraint.validate(&value).is_err());
    }
}
