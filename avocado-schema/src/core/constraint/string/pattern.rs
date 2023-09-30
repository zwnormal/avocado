use crate::base::{SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use regex::Regex;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub struct Pattern {
    pub pattern: Regex,
}

impl Serialize for Pattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.pattern.as_str())
    }
}

impl<'de> Deserialize<'de> for Pattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(PatternVisitor)
    }
}

struct PatternVisitor;

impl<'de> Visitor<'de> for PatternVisitor {
    type Value = Pattern;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "string field [pattern] needs to be a valid regular expression"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let pattern = Regex::new(v).map_err(|e| Error::custom(e.to_string()))?;
        Ok(Pattern { pattern })
    }
}

impl Constraint for Pattern {
    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::String(v) if !self.pattern.is_match(v.as_str()) => {
                Err(SchemaError::Verification {
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
            pattern: r"^\d{4}-\d{2}-\d{2}$".parse().unwrap(),
        };

        let value = Value::String("2010-03-14".to_string());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("Not Match".to_string());
        assert!(constraint.validate(&value).is_err());
    }
}
