use crate::base::field::{Field, FieldType};
use crate::core::constraint::common::typed::Type;
use crate::core::constraint::string::enumeration::Enumeration;
use crate::core::constraint::string::max_length::MaxLength;
use crate::core::constraint::string::min_length::MinLength;
use crate::core::constraint::string::pattern::Pattern;
use crate::core::constraint::Constraint;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "string")]
pub struct StringField {
    pub name: String,
    pub title: String,
    #[serde(flatten)]
    pub enumeration: Option<Enumeration>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<MaxLength>,
    #[serde(rename = "minLength")]
    pub min_length: Option<MinLength>,
    pub pattern: Option<Pattern>,
}

impl Field for StringField {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn get_type(&self) -> FieldType {
        FieldType::String
    }

    fn into_enum(self) -> crate::base::visitor::Field {
        crate::base::visitor::Field::String(self)
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        let mut constraints: Vec<Box<dyn Constraint>> = vec![Box::new(Type {
            typed: FieldType::String,
        })];
        if self.enumeration.is_some() {
            constraints.push(Box::new(self.enumeration.as_ref().unwrap().clone()))
        }
        if self.max_length.is_some() {
            constraints.push(Box::new(self.max_length.as_ref().unwrap().clone()))
        }
        if self.min_length.is_some() {
            constraints.push(Box::new(self.min_length.as_ref().unwrap().clone()))
        }
        if self.pattern.is_some() {
            constraints.push(Box::new(self.pattern.as_ref().unwrap().clone()))
        }
        constraints
    }
}

#[derive(Default)]
pub struct StringFieldBuilder {
    name: String,
    title: String,
    enumeration: Option<Vec<String>>,
    max_length: Option<usize>,
    min_length: Option<usize>,
    pattern: Option<Regex>,
}

impl StringFieldBuilder {
    pub fn new() -> Self {
        StringFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn enumeration(mut self, strings: Vec<String>) -> Self {
        self.enumeration = Some(strings);
        self
    }

    pub fn max_length(mut self, length: usize) -> Self {
        self.max_length = Some(length);
        self
    }

    pub fn min_length(mut self, length: usize) -> Self {
        self.min_length = Some(length);
        self
    }

    pub fn pattern(mut self, pattern: Regex) -> Self {
        self.pattern = Some(pattern);
        self
    }

    pub fn build(self) -> StringField {
        StringField {
            name: self.name,
            title: self.title,
            enumeration: self.enumeration.map(|values| Enumeration { values }),
            max_length: self.max_length.map(|max_length| MaxLength { max_length }),
            min_length: self.min_length.map(|min_length| MinLength { min_length }),
            pattern: self.pattern.map(|pattern| Pattern { pattern }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::string::{StringField, StringFieldBuilder};
    use regex::Regex;

    #[test]
    fn test_serialize() {
        let field = StringFieldBuilder::new()
            .name("subtype")
            .title("SubType")
            .enumeration(vec!["meeting".to_string(), "email".to_string()])
            .max_length(32)
            .min_length(8)
            .pattern(Regex::new(r"[a-z]+").unwrap())
            .build();
        let field_json = serde_json::to_string(&field).unwrap();
        assert_eq!(
            field_json,
            r#"{"type":"string","name":"subtype","title":"SubType","enum":["meeting","email"],"maxLength":32,"minLength":8,"pattern":"[a-z]+"}"#
        );
    }

    #[test]
    fn test_deserialize() {
        let field_json = r#"
        {
            "type":"string",
            "name": "subtype",
            "title": "SubType",
            "enum": ["meeting", "email"],
            "maxLength": 32,
            "minLength": 8,
            "pattern": "[a-z]+"
        }"#;
        let field: StringField = serde_json::from_str(field_json).unwrap();
        assert_eq!(field.name, "subtype");
        assert_eq!(field.title, "SubType");
        assert_eq!(field.enumeration.unwrap().values, vec!["meeting", "email"]);
        assert_eq!(field.max_length.unwrap().max_length, 32);
        assert_eq!(field.min_length.unwrap().min_length, 8);
        assert_eq!(field.pattern.unwrap().pattern.to_string(), "[a-z]+");
    }
}
