use crate::base::Field;
use crate::core::constraint::object::required::Required;
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
}
