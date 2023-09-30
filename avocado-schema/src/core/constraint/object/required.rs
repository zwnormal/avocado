use crate::base::{SchemaError, SchemaResult};
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Required {
    pub required: Vec<String>,
}

impl Constraint for Required {
    fn validate(&self, val: &Value) -> SchemaResult {
        match val {
            Value::Object(o) => {
                let mut missing_fields = vec![];
                for field in &self.required {
                    match o.get(field.as_str()) {
                        None => {
                            missing_fields.push(field.clone());
                        }
                        Some(v) if v == &Value::Null => {
                            missing_fields.push(field.clone());
                        }
                        _ => {}
                    }
                }

                if !missing_fields.is_empty() {
                    Err(SchemaError::Verification {
                        message: format!("[{}] field(s) are required", missing_fields.join(", ")),
                        constraint_name: "Required".to_string(),
                    })
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::core::constraint::object::required::Required;
    use crate::core::constraint::Constraint;
    use serde::Serialize;

    #[test]
    fn test_required() {
        #[derive(Serialize)]
        struct Document {
            title: String,
        }

        let document = serde_json::to_value(Document {
            title: "Document Title".to_string(),
        })
        .expect("failed to serialise document");

        let constraint = Required {
            required: vec!["title".to_string()],
        };
        assert!(constraint.validate(&document).is_ok());

        let constraint = Required {
            required: vec!["title".to_string(), "body".to_string()],
        };
        assert!(constraint.validate(&document).is_err());
    }
}
