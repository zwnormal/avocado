use crate::base::{Field, FieldType};
use crate::core::constraint::string::enumeration::Enumeration;
use crate::core::constraint::string::max_length::MaxLength;
use crate::core::constraint::string::min_length::MinLength;
use crate::core::constraint::string::pattern::Pattern;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StringField {
    pub name: String,
    pub title: String,
    #[serde(flatten)]
    pub enumeration: Option<Enumeration>,
    #[serde(flatten)]
    pub max_length: Option<MaxLength>,
    #[serde(flatten)]
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

#[cfg(test)]
mod tests {
    use crate::core::constraint::string::enumeration::Enumeration;
    use crate::core::constraint::string::max_length::MaxLength;
    use crate::core::constraint::string::min_length::MinLength;
    use crate::core::constraint::string::pattern::Pattern;
    use crate::core::string::StringField;
    use regex::Regex;

    #[test]
    fn test_serialize() {
        let field = StringField {
            name: "type".to_string(),
            title: "Type".to_string(),
            enumeration: Some(Enumeration {
                values: vec!["meeting".to_string(), "email".to_string()],
            }),
            max_length: Some(MaxLength { max_length: 32 }),
            min_length: Some(MinLength { min_length: 8 }),
            pattern: Some(Pattern {
                pattern: Regex::new(r"[a-z]+").unwrap(),
            }),
        };
        let field_json = serde_json::to_string(&field).unwrap();
        assert_eq!(
            field_json,
            r#"{"name":"type","title":"Type","enum":["meeting","email"],"maxLength":32,"minLength":8,"pattern":"[a-z]+"}"#
        );
    }

    #[test]
    fn test_deserialize() {
        let field_json = r#"
        {
            "name": "type",
            "title": "Type",
            "enum": ["meeting", "email"],
            "maxLength": 32,
            "minLength": 8,
            "pattern": "[a-z]+"
        }"#;
        let field: StringField = serde_json::from_str(field_json).unwrap();
        assert_eq!(field.name, "type");
        assert_eq!(field.title, "Type");
        assert_eq!(field.enumeration.unwrap().values, vec!["meeting", "email"]);
        assert_eq!(field.max_length.unwrap().max_length, 32);
        assert_eq!(field.min_length.unwrap().min_length, 8);
        assert_eq!(field.pattern.unwrap().pattern.to_string(), "[a-z]+");
    }
}
