use crate::base::field::{Field, FieldType};
use crate::base::visitor::FieldEnum;
use crate::core::constraint::common::typed::Type;
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayField {
    pub name: String,
    pub title: String,
    pub item: Arc<FieldEnum>,
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
