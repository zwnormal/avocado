use crate::base::{SchemaError, SchemaResult, Value};
use crate::core::constraint::Constraint;

#[derive(Debug)]
pub struct ExclusiveMaximum {
    pub max_val: i64,
}

impl Constraint for ExclusiveMaximum {
    fn verify(&self) -> SchemaResult {
        Ok(())
    }

    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::Integer(v) if *v >= self.max_val => Err(SchemaError::VerificationFailed {
                message: format!("The {} is larger then or equals to {}", v, self.max_val),
                constraint_name: "ExclusiveMaximum".to_string(),
            }),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::Value;
    use crate::core::constraint::integer::exclusive_maximum::ExclusiveMaximum;
    use crate::core::constraint::Constraint;

    #[test]
    fn test_exclusive_maximum() {
        let constraint = ExclusiveMaximum { max_val: 10 };

        let value = Value::Integer(3);
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Integer(10);
        assert!(constraint.validate(&value).is_err());

        let value = Value::Integer(11);
        assert!(constraint.validate(&value).is_err());
    }
}
