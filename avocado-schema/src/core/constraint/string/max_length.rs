use crate::base::{SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use serde_json::Value;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct MaxLength {
    pub max_length: usize,
}

impl Constraint for MaxLength {
    fn verify(&self) -> SchemaResult {
        if self.max_length == 0 {
            Err(SchemaError::Verify {
                message: "The max length is 0".to_string(),
                constraint_name: "MaxLength".to_string(),
            })
        } else {
            Ok(())
        }
    }

    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::String(v) if v.graphemes(true).count() > self.max_length => {
                Err(SchemaError::Verification {
                    message: format!(
                        "The length of {} is larger then {}",
                        v,
                        v.graphemes(true).count()
                    ),
                    constraint_name: "MaxLength".to_string(),
                })
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::string::max_length::MaxLength;
    use crate::core::constraint::Constraint;
    use serde_json::Value;

    #[test]
    fn test_max_length() {
        let constraint = MaxLength { max_length: 6 };

        let value = Value::String("Valid".to_string());
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("Invalid String".to_string());
        assert!(constraint.validate(&value).is_err());

        let constraint = MaxLength { max_length: 0 };
        assert!(constraint.verify().is_err());
    }
}
