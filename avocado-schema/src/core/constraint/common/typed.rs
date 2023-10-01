use crate::base::field::FieldType;
use crate::base::{SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    #[serde(rename = "type")]
    pub typed: FieldType,
}

impl Constraint for Type {
    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::Bool(_) if matches!(self.typed, FieldType::Boolean) => Ok(()),
            Value::Number(n) if matches!(self.typed, FieldType::Integer) && n.is_i64() => Ok(()),
            Value::Number(n) if matches!(self.typed, FieldType::Float) && n.is_f64() => Ok(()),
            Value::String(_) if matches!(self.typed, FieldType::String) => Ok(()),
            Value::Array(_) if matches!(self.typed, FieldType::Array) => Ok(()),
            Value::Null => Ok(()),
            _ => Err(SchemaError::Validation {
                message: format!("The value {} is not type {}", val, self.typed),
                constraint_name: "Type".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::field::FieldType;
    use crate::core::constraint::common::typed::Type;
    use crate::core::constraint::Constraint;
    use serde_json::Value;

    #[test]
    fn validate_boolean() {
        let constraint = Type {
            typed: FieldType::Boolean,
        };

        let value = Value::Bool(true);
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("Test".to_string());
        assert!(constraint.validate(&value).is_err())
    }
}
