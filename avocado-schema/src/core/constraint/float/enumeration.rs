use crate::core::constraint::Constraint;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Enumeration {
    #[serde(rename = "enum")]
    pub values: Vec<f64>,
}

impl Constraint for Enumeration {
    fn validate(&self, val: &Value) -> Result<()> {
        match val {
            Value::Number(v) if v.is_f64() && !self.values.contains(&v.as_f64().unwrap()) => {
                Err(anyhow!(format!(
                    "value {} is not valid value ({})",
                    v, "Enum of Float"
                )))
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::float::enumeration::Enumeration;
    use crate::core::constraint::Constraint;
    use serde_json::{Number, Value};

    #[test]
    fn test_enumeration() {
        let constraint = Enumeration {
            values: vec![1.1, 2.5],
        };

        let value = Value::Number(Number::from_f64(1.1).unwrap());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(Number::from_f64(3.0).unwrap());
        assert!(constraint.validate(&value).is_err());
    }
}
