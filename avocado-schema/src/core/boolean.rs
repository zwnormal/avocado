use crate::base::Field;
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
}
