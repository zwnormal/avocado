use crate::base::{Field, FieldType};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ObjectField {
    pub name: String,
    pub title: String,
    pub fields: HashMap<String, Box<dyn Field>>,
}

impl ObjectField {
    const TYPE: FieldType = FieldType::Object;

    pub fn new(name: &'static str, title: &'static str) -> Self {
        ObjectField {
            name: name.to_string(),
            title: title.to_string(),
            fields: HashMap::new(),
        }
    }

    pub fn add_field(
        &mut self,
        name: &'static str,
        field: impl Field + 'static,
    ) -> Result<(), String> {
        if self.fields.contains_key(name) {
            return Err(format!("The field with name {} already exists", name));
        }
        self.fields.insert(name.to_string(), Box::new(field));
        Ok(())
    }
}

impl Field for ObjectField {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn get_type(&self) -> FieldType {
        Self::TYPE
    }
}
