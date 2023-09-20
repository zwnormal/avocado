use crate::base::{Field, FieldType};

#[derive(Debug)]
pub struct StringField {
    pub name: String,
    pub title: String,
}

impl StringField {
    const TYPE: FieldType = FieldType::String;

    pub fn new(name: &'static str, title: &'static str) -> Self {
        StringField {
            name: name.to_string(),
            title: title.to_string(),
        }
    }
}

impl Field for StringField {
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
