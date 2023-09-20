use crate::base::Value;
use crate::core::constraint::Constraint;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct MaxLength {
    pub max_length: usize,
}

impl Constraint for MaxLength {
    fn verify(&self) -> Result<(), String> {
        if self.max_length == 0 {
            Err("The max length is 0".to_string())
        } else {
            Ok(())
        }
    }

    fn validate(&self, val: &Value) -> Result<(), String> {
        match val {
            Value::String(v) if v.graphemes(true).count() > self.max_length => Err(format!(
                "The length of {} is larger then {} (MaxLength)",
                v,
                v.graphemes(true).count()
            )),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::Value;
    use crate::core::constraint::string::max_length::MaxLength;
    use crate::core::constraint::Constraint;

    #[test]
    fn test_max_length() {
        let constraint = MaxLength { max_length: 6 };

        let value = Value::String("Valid".to_string());
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("Invalid String".to_string());
        assert!(constraint.validate(&value).is_err());

        let constraint = MaxLength { max_length: 0 };
        assert!(constraint.verify().is_err());
    }
}
