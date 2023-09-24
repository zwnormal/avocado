use crate::base::{Field, FieldType};
use crate::core::constraint::string::enumeration::Enumeration;
use crate::core::constraint::string::max_length::MaxLength;
use crate::core::constraint::string::min_length::MinLength;
use crate::core::constraint::string::pattern::Pattern;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct StringField {
    pub name: String,
    pub title: String,
    pub enumeration: Option<Enumeration>,
    pub max_length: Option<MaxLength>,
    pub min_length: Option<MinLength>,
    pub pattern: Option<Pattern>,
}

impl StringField {
    const TYPE: FieldType = FieldType::String;
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
