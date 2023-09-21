use crate::base::{SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use regex::Regex;
use serde_json::Value;

#[derive(Debug)]
pub struct Pattern {
    pub pattern: String,
}

impl Pattern {
    fn regex(&self) -> Result<Regex, SchemaError> {
        Ok(
            Regex::new(self.pattern.as_str()).map_err(|e| SchemaError::VerifyFailed {
                message: e.to_string(),
                constraint_name: "Pattern".to_string(),
            })?,
        )
    }
}

impl Constraint for Pattern {
    fn verify(&self) -> SchemaResult {
        self.regex()?;
        Ok(())
    }

    fn validate(&self, val: &Value) -> SchemaResult {
        let regex = self.regex()?;
        match val {
            Value::String(v) if !regex.is_match(v.as_str()) => {
                Err(SchemaError::VerificationFailed {
                    message: format!("{} does not match pattern {}", self.pattern, v),
                    constraint_name: "Pattern".to_string(),
                })
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::string::pattern::Pattern;
    use crate::core::constraint::Constraint;
    use serde_json::Value;

    #[test]
    fn test_pattern() {
        let constraint = Pattern {
            pattern: r"^\d{4}-\d{2}-\d{2}$".to_string(),
        };

        let value = Value::String("2010-03-14".to_string());
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("Not Match".to_string());
        assert!(constraint.validate(&value).is_err());

        let constraint = Pattern {
            pattern: "[".to_string(),
        };
        assert!(constraint.verify().is_err());
    }
}
