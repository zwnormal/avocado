use crate::base::field::{Field, FieldType};
use crate::base::visitor::FieldEnum;
use crate::base::visitor::Visitor;
use crate::base::SchemaError;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct Validator {
    pub value: Value,
    pub field_names: Vec<String>,
    pub errors: HashMap<String, Vec<SchemaError>>,
}

impl Validator {
    fn report_error(&mut self, error: SchemaError) {
        let field = self.field_names.clone().join("/");
        if self.errors.contains_key(field.as_str()) {
            self.errors.get_mut(field.as_str()).unwrap().push(error);
        } else {
            self.errors.insert(field, vec![error]);
        }
    }

    fn validate(&mut self, field: &(impl Field + ?Sized)) {
        for constraint in field.constrains() {
            match constraint.validate(&self.value) {
                Ok(_) => {}
                Err(e) => {
                    self.report_error(e);
                }
            }
        }
    }
}

impl Visitor for Validator {
    fn visit(&mut self, field: Arc<FieldEnum>) {
        match field.as_ref() {
            FieldEnum::Array(f) => {
                self.field_names.push(f.name.clone());
                self.validate(f);
                match self.value.clone() {
                    Value::Array(values) => {
                        for value in values {
                            self.value = value;
                            self.visit(f.item.clone());
                        }
                    }
                    _ => {
                        self.report_error(SchemaError::Validation {
                            message: format!(
                                "The value {} is not type {}",
                                self.value,
                                FieldType::Array
                            ),
                            constraint_name: "Type".to_string(),
                        });
                    }
                }
                self.field_names.pop();
            }
            FieldEnum::Boolean(f) => {
                self.field_names.push(f.name.clone());
                self.validate(f);
                self.field_names.pop();
            }
            FieldEnum::Float(f) => {
                self.field_names.push(f.name.clone());
                self.validate(f);
                self.field_names.pop();
            }
            FieldEnum::Integer(f) => {
                self.field_names.push(f.name.clone());
                self.validate(f);
                self.field_names.pop();
            }
            FieldEnum::Object(f) => {
                self.field_names.push(f.name.clone());
                self.validate(f);
                match self.value.clone() {
                    Value::Object(o) => {
                        for (name, value) in o {
                            match f.properties.get(name.as_str()) {
                                Some(field) => {
                                    self.value = value;
                                    self.visit(field.clone());
                                }
                                None => {}
                            };
                        }
                    }
                    _ => {
                        self.report_error(SchemaError::Validation {
                            message: format!(
                                "The value {} is not type {}",
                                self.value,
                                FieldType::Object
                            ),
                            constraint_name: "Type".to_string(),
                        });
                    }
                }
                self.field_names.pop();
            }
            FieldEnum::String(f) => {
                self.field_names.push(f.name.clone());
                self.validate(f);
                self.field_names.pop();
            }
        }
    }
}
