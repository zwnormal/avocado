use crate::base::{Field, FieldType};
use crate::core::constraint::object::required::Required;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ObjectField {
    pub name: String,
    pub title: String,
    pub properties: HashMap<String, Box<dyn Field>>,
    pub required: Option<Required>,
}

impl ObjectField {
    const TYPE: FieldType = FieldType::Object;
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
