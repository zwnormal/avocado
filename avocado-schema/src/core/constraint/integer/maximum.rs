use crate::base::Value;
use crate::core::constraint::Constraint;

#[derive(Debug)]
pub struct Maximum {
    pub max_val: i64,
}

impl Constraint for Maximum {
    fn verify(&self) -> Result<(), String> {
        Ok(())
    }

    fn validate(&self, val: &Value) -> Result<(), String> {
        match val {
            Value::Integer(v) if *v > self.max_val => Err(format!(
                "The {} is larger then {} (Maximum)",
                v, self.max_val
            )),
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
