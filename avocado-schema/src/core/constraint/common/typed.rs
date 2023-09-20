use crate::base::{FieldType, Value};
use crate::core::constraint::Constraint;

#[derive(Debug)]
pub struct Type {
    pub typed: FieldType,
}

impl Constraint for Type {
    fn verify(&self) -> Result<(), String> {
        Ok(())
    }

    fn validate(&self, val: &Value) -> Result<(), String> {
        match val {
            Value::Boolean(_) if matches!(self.typed, FieldType::Boolean) => Ok(()),
            Value::Integer(_) if matches!(self.typed, FieldType::Integer) => Ok(()),
            Value::String(_) if matches!(self.typed, FieldType::String) => Ok(()),
            Value::Array(_) if matches!(self.typed, FieldType::Array) => Ok(()),
            Value::Null => Ok(()),
            _ => Err(format!(
                "The value {} is not type {} (Type)",
                val, self.typed
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::{FieldType, Value};
    use crate::core::constraint::common::typed::Type;
    use crate::core::constraint::Constraint;

    #[test]
    fn validate_boolean() {
        let constraint = Type {
            typed: FieldType::Boolean,
        };

        let value = Value::Boolean(true);
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("Test".to_string());
        assert!(constraint.validate(&value).is_err())
    }
}