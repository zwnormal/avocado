use crate::base::{SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt::Formatter;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug)]
pub struct MaxLength {
    pub max_length: usize,
}

impl Serialize for MaxLength {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.max_length as u64)
    }
}

impl<'de> Deserialize<'de> for MaxLength {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(MaxLengthVisitor)
    }
}

struct MaxLengthVisitor;

impl<'de> Visitor<'de> for MaxLengthVisitor {
    type Value = MaxLength;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "string field [maxLength] is invalid")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let max_length = usize::try_from(v).map_err(|e| Error::custom(e.to_string()))?;
        if max_length == 0 {
            Err(Error::custom("string field [maxLength] equals to 0"))
        } else {
            Ok(MaxLength { max_length })
        }
    }
}

impl Constraint for MaxLength {
    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::String(v)
                if v.graphemes(true).count()
                    > usize::try_from(self.max_length).map_err(|_| SchemaError::Verify {
                        message: "The max length is too larger".to_string(),
                        constraint_name: "MaxLength".to_string(),
                    })? =>
            {
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
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("Invalid String".to_string());
        assert!(constraint.validate(&value).is_err());
    }
}
