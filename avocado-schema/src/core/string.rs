use crate::base::{Field, FieldType};
use crate::core::constraint::string::enumeration::Enumeration;
use crate::core::constraint::string::max_length::MaxLength;
use crate::core::constraint::string::min_length::MinLength;
use crate::core::constraint::string::pattern::Pattern;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Debug)]
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

impl Serialize for StringField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("StringField", 6)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("enum", &self.enumeration.clone().map(|e| e.values))?;
        state.serialize_field(
            "max_length",
            &self.max_length.as_ref().map(|m| m.max_length),
        )?;
        state.serialize_field(
            "min_length",
            &self.min_length.as_ref().map(|m| m.min_length),
        )?;
        state.serialize_field("pattern", &self.pattern.clone().map(|p| p.pattern))?;
        state.end()
    }
}
