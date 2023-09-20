use crate::base::{Field, FieldType};

#[derive(Debug)]
pub struct IntegerField {
    pub name: String,
    pub title: String,
}

impl IntegerField {
    const TYPE: FieldType = FieldType::Integer;

    pub fn new(name: &'static str, title: &'static str) -> Self {
        IntegerField {
            name: name.to_string(),
            title: title.to_string(),
        }
    }
}

impl Field for IntegerField {
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
