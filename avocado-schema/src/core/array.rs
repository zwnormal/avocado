use crate::base::field::{Field, FieldType};
use crate::core::constraint::common::typed::Type;
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "array")]
pub struct ArrayField {
    pub name: String,
    pub title: String,
    pub item: Arc<crate::base::visitor::Field>,
}

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
}
