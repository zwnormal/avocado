use crate::base::{SchemaError, SchemaResult, Value};
use crate::core::constraint::Constraint;

#[derive(Debug)]
pub struct Maximum {
    pub max_val: i64,
}

impl Constraint for Maximum {
    fn verify(&self) -> SchemaResult {
        Ok(())
    }

    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::Integer(v) if *v > self.max_val => Err(SchemaError::VerificationFailed {
                message: format!("The {} is larger then {}", v, self.max_val),
                constraint_name: "Maximum".to_string(),
            }),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::Value;
    use crate::core::constraint::integer::maximum::Maximum;
    use crate::core::constraint::Constraint;

    #[test]
    fn test_maximum() {
        let constraint = Maximum { max_val: 10 };

        let value = Value::Integer(3);
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Integer(10);
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Integer(11);
        assert!(constraint.validate(&value).is_err());
    }
}
