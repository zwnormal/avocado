use crate::base::Value;
use crate::core::constraint::Constraint;

#[derive(Debug)]
pub struct ExclusiveMinimum {
    pub min_val: i64,
}

impl Constraint for ExclusiveMinimum {
    fn verify(&self) -> Result<(), String> {
        Ok(())
    }

    fn validate(&self, val: &Value) -> Result<(), String> {
        match val {
            Value::Integer(v) if *v <= self.min_val => Err(format!(
                "The {} is less then or equals to {} (ExclusiveMinimum)",
                v, self.min_val
            )),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::Value;
    use crate::core::constraint::integer::exclusive_minimum::ExclusiveMinimum;
    use crate::core::constraint::Constraint;

    #[test]
    fn test_exclusive_minimum() {
        let constraint = ExclusiveMinimum { min_val: 10 };

        let value = Value::Integer(11);
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Integer(10);
        assert!(constraint.validate(&value).is_err());

        let value = Value::Integer(3);
        assert!(constraint.validate(&value).is_err());
    }
}
