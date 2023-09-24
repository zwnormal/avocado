use crate::base::{SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use serde::{Serialize, Serializer};
use serde_json::Value;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct MinLength {
    pub min_length: u64,
}

impl Serialize for MinLength {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.min_length)
    }
}

impl Constraint for MinLength {
    fn verify(&self) -> SchemaResult {
        Ok(())
    }

    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::String(v) if v.graphemes(true).count() < self.min_length as usize => {
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
