use crate::core::constraint::Constraint;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExclusiveMinimum {
    #[serde(rename = "exclusiveMinimum")]
    pub min_val: i64,
}

impl Constraint for ExclusiveMinimum {
    fn validate(&self, val: &Value) -> Result<()> {
        match val {
            Value::Number(v) if v.is_i64() && (v.as_i64().unwrap() <= self.min_val) => {
                Err(anyhow!(format!(
                    "value {} is less then or equals to {} ({})",
                    v, self.min_val, "ExclusiveMinimum"
                )))
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::integer::exclusive_minimum::ExclusiveMinimum;
    use crate::core::constraint::Constraint;
    use serde_json::Value;

    #[test]
    fn test_exclusive_minimum() {
        let constraint = ExclusiveMinimum { min_val: 10 };

        let value = Value::Number(11.into());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(10.into());
        assert!(constraint.validate(&value).is_err());

        let value = Value::Number(3.into());
        assert!(constraint.validate(&value).is_err());
    }
}
