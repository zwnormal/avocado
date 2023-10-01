use crate::base::field::{Field, FieldType};
use crate::core::constraint::common::typed::Type;
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "boolean")]
pub struct BooleanField {
    pub name: String,
    pub title: String,
}

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
}

#[derive(Default)]
pub struct BooleanFieldBuilder {
    name: String,
    title: String,
}

impl BooleanFieldBuilder {
    pub fn new() -> Self {
        BooleanFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn build(self) -> BooleanField {
        BooleanField {
            name: self.name,
            title: self.title,
        }
    }
}
