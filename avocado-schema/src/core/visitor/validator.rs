use crate::base::field::{Field, FieldType};
use crate::base::visitor::Visitor;
use crate::base::SchemaError;
use crate::core::array::ArrayField;
use crate::core::boolean::BooleanField;
use crate::core::float::FloatField;
use crate::core::integer::IntegerField;
use crate::core::object::ObjectField;
use crate::core::string::StringField;
use serde_json::Value;
use std::collections::HashMap;

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
    fn visit_array(&mut self, array: &ArrayField) {
        self.field_names.push(array.name.clone());
        self.validate(array);
        match self.value.clone() {
            Value::Array(values) => {
                for value in values {
                    self.value = value.clone();
                    self.validate(&*array.item);
                }
            }
            _ => {
                self.report_error(SchemaError::Validation {
                    message: format!("The value {} is not type {}", self.value, FieldType::Array),
                    constraint_name: "Type".to_string(),
                });
            }
        }
        self.field_names.pop();
    }

    fn visit_boolean(&mut self, boolean: &BooleanField) {
        self.field_names.push(boolean.name.clone());
        self.validate(boolean);
        self.field_names.pop();
    }

    fn visit_float(&mut self, float: &FloatField) {
        self.field_names.push(float.name.clone());
        self.validate(float);
        self.field_names.pop();
    }

    fn visit_integer(&mut self, integer: &IntegerField) {
        self.field_names.push(integer.name.clone());
        self.validate(integer);
        self.field_names.pop();
    }

    fn visit_object(&mut self, object: &ObjectField) {
        todo!()
    }

    fn visit_string(&mut self, string: &StringField) {
        self.field_names.push(string.name.clone());
        self.validate(string);
        self.field_names.pop();
    }
}
