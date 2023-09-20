use crate::base::{SchemaError, SchemaResult, Value};
use crate::core::constraint::Constraint;

#[derive(Debug)]
pub struct Enumeration {
    pub values: Vec<i64>,
}

impl Constraint for Enumeration {
    fn verify(&self) -> SchemaResult {
        if self.values.len() == 0 {
            Err(SchemaError::VerifyFailed {
                message: "Can not be empty".to_string(),
                constraint_name: "Enum of Integer".to_string(),
            })
        } else {
            Ok(())
        }
    }

    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::Integer(v) if !self.values.contains(v) => Err(SchemaError::VerificationFailed {
                message: format!("The integer {} is not valid value", v),
                constraint_name: "Enum of Integer".to_string(),
            }),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::Value;
    use crate::core::constraint::integer::enumeration::Enumeration;
    use crate::core::constraint::Constraint;

    #[test]
    fn test_enumeration() {
        let constraint = Enumeration { values: vec![1, 2] };

        let value = Value::Integer(1);
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Integer(3);
        assert!(constraint.validate(&value).is_err());

        let constraint = Enumeration { values: vec![] };
        assert!(constraint.verify().is_err());
    }
}
