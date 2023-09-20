use crate::base::{Field, FieldType};

#[derive(Debug)]
pub struct BooleanField {
    pub name: String,
    pub title: String,
}

impl BooleanField {
    const TYPE: FieldType = FieldType::Boolean;

    pub fn new(name: &'static str, title: &'static str) -> Self {
        BooleanField {
            name: name.to_string(),
            title: title.to_string(),
        }
    }
}

impl Field for BooleanField {
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
