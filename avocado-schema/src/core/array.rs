use crate::base::field::{Field, FieldType};
use crate::core::constraint::common::typed::Type;
use crate::core::constraint::Constraint;
use anyhow::{anyhow, Result};
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

    fn get_type(&self) -> FieldType {
        FieldType::Array
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        vec![Box::new(Type {
            typed: FieldType::Array,
        })]
    }
}

#[derive(Default)]
pub struct ArrayFieldBuilder {
    name: String,
    title: String,
    item: Option<crate::base::visitor::Field>,
}

impl ArrayFieldBuilder {
    pub fn new() -> ArrayFieldBuilder {
        ArrayFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn item(mut self, item: crate::base::visitor::Field) -> Self {
        self.item = Some(item);
        self
    }

    pub fn build(self) -> Result<ArrayField> {
        if let Some(item) = self.item {
            let field = ArrayField {
                name: self.name,
                title: self.title,
                item: Arc::new(item),
            };
            Ok(field)
        } else {
            Err(anyhow!("array field must specify item field"))
        }
    }
}
