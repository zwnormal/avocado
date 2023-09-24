use crate::base::{SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub struct Enumeration {
    pub values: Vec<String>,
}

impl Serialize for Enumeration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.values.len()))?;
        for element in &self.values {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Enumeration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(EnumerationVisitor)
    }
}

struct EnumerationVisitor;

impl<'de> Visitor<'de> for EnumerationVisitor {
    type Value = Enumeration;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "string field [enum] needs to be a vector of strings"
        )
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values: Vec<String> = vec![];
        while let Some(value) = seq.next_element()? {
            values.push(value);
        }
        Ok(Enumeration { values })
    }
}

impl Constraint for Enumeration {
    fn verify(&self) -> SchemaResult {
        if self.values.len() == 0 {
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
