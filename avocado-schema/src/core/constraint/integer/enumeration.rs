use crate::base::Value;
use crate::core::constraint::Constraint;

#[derive(Debug)]
pub struct Enumeration {
    pub values: Vec<i64>,
}

impl Constraint for Enumeration {
    fn verify(&self) -> Result<(), String> {
        if self.values.len() == 0 {
            Err("Can not be empty (Enum of Integer)".to_string())
        } else {
            Ok(())
        }
    }

    fn validate(&self, val: &Value) -> Result<(), String> {
        match val {
            Value::Integer(v) if !self.values.contains(v) => Err(format!(
                "The integer {} is not valid value (Enum of Integer)",
                v
            )),
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
