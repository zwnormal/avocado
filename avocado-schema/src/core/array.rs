use crate::base::Field;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayField {
    pub name: String,
    pub title: String,
    pub items: Box<dyn Field>,
}

#[typetag::serde(name = "array")]
impl Field for ArrayField {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }
}
