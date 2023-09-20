use crate::base::Value;
use crate::core::constraint::Constraint;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct MinLength {
    pub min_length: usize,
}

impl Constraint for MinLength {
    fn verify(&self) -> Result<(), String> {
        Ok(())
    }

    fn validate(&self, val: &Value) -> Result<(), String> {
        match val {
            Value::String(v) if v.graphemes(true).count() < self.min_length => Err(format!(
                "The length of {} is less then {} (MinLength)",
                v, self.min_length
            )),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::Value;
    use crate::core::constraint::string::min_length::MinLength;
    use crate::core::constraint::Constraint;

    #[test]
    fn test_min_length() {
        let constraint = MinLength { min_length: 8 };

        let value = Value::String("Valid String".to_string());
        assert!(constraint.verify().is_ok());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("Invalid".to_string());
        assert!(constraint.validate(&value).is_err());
    }
}
