use crate::base::{Field, FieldType};

#[derive(Debug)]
pub struct ArrayField<T: Field> {
    pub name: String,
    pub title: String,
    pub items: T,
}

impl<T: Field> ArrayField<T> {
    const TYPE: FieldType = FieldType::Array;
}

impl<T: Field> Field for ArrayField<T> {
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
