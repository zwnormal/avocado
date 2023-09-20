use crate::base::Value;
use crate::core::constraint::Constraint;

#[derive(Debug)]
pub struct Required {
    pub required: bool,
}

impl Constraint for Required {
    fn verify(&self) -> Result<(), String> {
        Ok(())
    }

    fn validate(&self, val: &Value) -> Result<(), String> {
        match val {
            Value::Null => {
                if self.required {
                    Err("The value is required (Required)".to_string())
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::Value;
    use crate::core::constraint::common::required::Required;
    use crate::core::constraint::Constraint;

    #[test]
    fn validate_required() {
        let constraint = Required { required: true };

        let value = Value::String("Test".to_string());
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Null;
        assert!(constraint.validate(&value).is_err())
    }
}
