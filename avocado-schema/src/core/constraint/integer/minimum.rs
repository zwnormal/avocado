use crate::base::{SchemaError, SchemaResult, Value};
use crate::core::constraint::Constraint;

#[derive(Debug)]
pub struct Minimum {
    pub min_val: i64,
}

impl Constraint for Minimum {
    fn verify(&self) -> SchemaResult {
        Ok(())
    }

    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::Integer(v) if *v < self.min_val => Err(SchemaError::VerificationFailed {
                message: format!("The {} is less then {}", v, self.min_val),
                constraint_name: "Minimum".to_string(),
            }),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::Value;
    use crate::core::constraint::integer::minimum::Minimum;
    use crate::core::constraint::Constraint;

    #[test]
    fn test_minimum() {
        let constraint = Minimum { min_val: 10 };

        let value = Value::Integer(11);
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Integer(10);
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Integer(3);
        assert!(constraint.validate(&value).is_err());
    }
}
