use crate::base::field::Field;
use crate::base::visitor::Field as FieldEnum;
use crate::base::visitor::Visitor;
use crate::base::SchemaError;
use crate::core::array::ArrayField;
use crate::core::object::ObjectField;
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
        self.field_names.push(field.name().clone());
        for constraint in field.constrains() {
            match constraint.validate(&self.value) {
                Ok(_) => {}
                Err(e) => {
                    self.report_error(e);
                }
            }
        }
        self.field_names.pop();
    }

    fn visit_array(&mut self, array: &ArrayField) {
        self.validate(array);
        if let Value::Array(values) = self.value.clone() {
            for value in values {
                self.value = value;
                self.visit(array.item.clone());
            }
        }
    }

    fn visit_object(&mut self, object: &ObjectField) {
        self.validate(object);
        if let Value::Object(o) = self.value.clone() {
            for (name, value) in o {
                if let Some(field) = object.properties.get(name.as_str()) {
                    self.value = value;
                    self.visit(field.clone());
                };
            }
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
                self.validate(f);
            }
            FieldEnum::Float(f) => {
                self.validate(f);
            }
            FieldEnum::Integer(f) => {
                self.validate(f);
            }
            FieldEnum::Object(f) => {
                self.visit_object(f);
            }
            FieldEnum::String(f) => {
                self.validate(f);
            }
        }
    }
}
