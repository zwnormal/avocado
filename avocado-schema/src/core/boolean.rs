use crate::base::field::{Field, FieldType};
use crate::base::visitor::Visitor as FieldVisitor;
use crate::core::constraint::common::typed::Type;
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BooleanField {
    pub name: String,
    pub title: String,
}

#[typetag::serde(name = "boolean")]
impl Field for BooleanField {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        vec![Box::new(Type {
            typed: FieldType::Boolean,
        })]
    }

    fn accept(&self, mut visitor: Box<dyn FieldVisitor>) {
        visitor.visit_boolean(self);
    }
}
