use crate::base::{Field, FieldType};

#[derive(Debug)]
pub struct ArrayField {
    pub name: String,
    pub title: String,
    pub items: Box<dyn Field>,
}

impl ArrayField {
    const TYPE: FieldType = FieldType::Array;
}

impl Field for ArrayField {
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
