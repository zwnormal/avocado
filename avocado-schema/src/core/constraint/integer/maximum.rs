use crate::core::constraint::Constraint;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Maximum {
    #[serde(rename = "maximum")]
    pub max_val: i64,
}

impl Constraint for Maximum {
    fn validate(&self, val: &Value) -> Result<()> {
        match val {
            Value::Number(v) if v.is_i64() && (v.as_i64().unwrap() > self.max_val) => {
                Err(anyhow!(format!(
                    "value {} is larger then {} ({})",
                    v, self.max_val, "Maximum"
                )))
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::integer::maximum::Maximum;
    use crate::core::constraint::Constraint;
    use serde_json::Value;

    #[test]
    fn test_maximum() {
        let constraint = Maximum { max_val: 10 };

        let value = Value::Number(3.into());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(10.into());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(11.into());
        assert!(constraint.validate(&value).is_err());
    }
}
