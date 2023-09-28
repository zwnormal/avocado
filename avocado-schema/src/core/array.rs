use crate::base::field::{Field, FieldType};
use crate::base::visitor::Visitor as FieldVisitor;
use crate::core::constraint::common::typed::Type;
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayField {
    pub name: String,
    pub title: String,
    pub item: Box<dyn Field>,
}

#[typetag::serde(name = "array")]
impl Field for ArrayField {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        vec![Box::new(Type {
            typed: FieldType::Array,
        })]
    }

    fn accept(&self, mut visitor: Box<dyn FieldVisitor>) {
        visitor.visit_array(self);
    }
}
