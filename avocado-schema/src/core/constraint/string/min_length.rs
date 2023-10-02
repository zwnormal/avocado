use crate::core::constraint::Constraint;
use anyhow::{anyhow, Result};
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt::Formatter;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug)]
pub struct MinLength {
    pub min_length: usize,
}

impl Serialize for MinLength {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.min_length as u64)
    }
}

impl<'de> Deserialize<'de> for MinLength {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(MinLengthVisitor)
    }
}

struct MinLengthVisitor;

impl<'de> Visitor<'de> for MinLengthVisitor {
    type Value = MinLength;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "string field [minLength] is invalid")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let min_length = usize::try_from(v).map_err(|e| Error::custom(e.to_string()))?;
        Ok(MinLength { min_length })
    }
}

impl Constraint for MinLength {
    fn validate(&self, val: &Value) -> Result<()> {
        match val {
            Value::String(v) if v.graphemes(true).count() < self.min_length => {
                Err(anyhow!(format!(
                    "length of {} is less then {} ({})",
                    v, self.min_length, "MinLength"
                )))
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
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("Invalid".to_string());
        assert!(constraint.validate(&value).is_err());
    }
}
