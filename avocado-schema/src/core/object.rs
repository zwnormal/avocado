use crate::base::field::{Field, FieldType};
use crate::base::visitor::Visitor as FieldVisitor;
use crate::core::constraint::common::typed::Type;
use crate::core::constraint::object::required::Required;
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectField {
    pub name: String,
    pub title: String,
    pub properties: HashMap<String, Box<dyn Field>>,
    pub required: Option<Required>,
}

#[typetag::serde(name = "object")]
impl Field for ObjectField {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        let mut constraints: Vec<Box<dyn Constraint>> = vec![Box::new(Type {
            typed: FieldType::Object,
        })];
        if self.required.is_some() {
            constraints.push(Box::new(self.required.as_ref().unwrap().clone()))
        }
        constraints
    }

    fn accept(&self, visitor: Box<dyn FieldVisitor>) {
        visitor.visit_object(self);
    }
}
