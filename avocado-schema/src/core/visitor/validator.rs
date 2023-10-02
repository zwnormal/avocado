use crate::base::field::Field;
use crate::base::visitor::Field as FieldEnum;
use crate::base::visitor::Visitor;
use crate::core::array::ArrayField;
use crate::core::object::ObjectField;
use anyhow::{anyhow, Error, Result};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct Validator {
    pub value: Value,
    pub field_names: Vec<String>,
    pub errors: HashMap<String, Vec<Error>>,
}

impl Validator {
    fn report_error(&mut self, error: Error) {
        let field = self.field_names.clone().join("/");
        if self.errors.contains_key(field.as_str()) {
            self.errors.get_mut(field.as_str()).unwrap().push(error);
        } else {
            self.errors.insert(field, vec![error]);
        }
    }

    fn validate_field(&mut self, field: &(impl Field + ?Sized)) {
        self.field_names.push(field.name().clone());
        for constraint in field.constrains() {
            match constraint.validate(&self.value) {
                Ok(_) => {}
                Err(e) => {
                    self.report_error(anyhow!(e.to_string()));
                }
            }
        }
        self.field_names.pop();
    }

    fn visit_array(&mut self, array: &ArrayField) {
        self.validate_field(array);
        if let Value::Array(values) = self.value.clone() {
            for value in values {
                self.value = value;
                self.visit(array.item.clone());
            }
        }
    }

    fn visit_object(&mut self, object: &ObjectField) {
        self.validate_field(object);
        if let Value::Object(o) = self.value.clone() {
            for (name, value) in o {
                if let Some(field) = object.properties.get(name.as_str()) {
                    self.value = value;
                    self.visit(field.clone());
                };
            }
        }
    }

    pub fn validate(
        value: &impl Serialize,
        field: Arc<FieldEnum>,
    ) -> Result<(), HashMap<String, Vec<Error>>> {
        let mut validator = Validator {
            value: serde_json::to_value(value).map_err(|e| {
                HashMap::from([("invalid value".to_string(), vec![anyhow!(e.to_string())])])
            })?,
            field_names: vec![],
            errors: Default::default(),
        };
        validator.visit(field);
        if validator.errors.is_empty() {
            Ok(())
        } else {
            Err(validator.errors)
        }
    }
}

impl Visitor for Validator {
    fn visit(&mut self, field: Arc<FieldEnum>) {
        match field.as_ref() {
            FieldEnum::Array(f) => {
                self.visit_array(f);
            }
            FieldEnum::Boolean(f) => {
                self.validate_field(f);
            }
            FieldEnum::Float(f) => {
                self.validate_field(f);
            }
            FieldEnum::Integer(f) => {
                self.validate_field(f);
            }
            FieldEnum::Object(f) => {
                self.visit_object(f);
            }
            FieldEnum::String(f) => {
                self.validate_field(f);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::field::Field;
    use crate::core::object::ObjectField;
    use crate::core::visitor::validator::Validator;
    use serde::Serialize;
    use std::sync::Arc;

    #[test]
    fn test_validate() {
        #[derive(Serialize)]
        struct Client {
            first_name: String,
            last_name: String,
            age: u64,
        }

        let schema_json = r#"
        {
            "type":"object",
            "name": "client",
            "title": "Client",
            "properties": {
                "first_name": {
                    "type": "string",
                    "name": "first_name",
                    "title": "First Name",
                    "max_length": 32,
                    "min_length": 8
                },
                "last_name": {
                    "type": "string",
                    "name": "last_name",
                    "title": "Last Name",
                    "max_length": 32,
                    "min_length": 8
                },
                "age": {
                    "type": "integer",
                    "name": "age",
                    "title": "Age",
                    "maximum": 200,
                    "minimum": 0
                }
            }
        }"#;
        let schema: ObjectField = serde_json::from_str(schema_json).unwrap();
        let schema = Arc::new(schema.into_enum());

        let valid_client = Client {
            first_name: "Robert".to_string(),
            last_name: "Li".to_string(),
            age: 32,
        };
        assert!(Validator::validate(&valid_client, schema.clone()).is_ok());

        let invalid_client = Client {
            first_name: "Robert".to_string(),
            last_name: "Li".to_string(),
            age: 201,
        };
        assert!(Validator::validate(&invalid_client, schema.clone()).is_err());
    }
}
