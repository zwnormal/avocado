use crate::base::{Field, FieldType};

#[derive(Debug)]
pub struct ArrayField<T: Field> {
    pub name: String,
    pub title: String,
    pub fields: Vec<T>,
}

impl<T: Field> ArrayField<T> {
    const TYPE: FieldType = FieldType::Array;

    pub fn new(name: &'static str, title: &'static str) -> Self {
        ArrayField {
            name: name.to_string(),
            title: title.to_string(),
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, field: T) -> Result<(), String> {
        self.fields.push(field);
        Ok(())
    }
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
