use crate::base::field::{Field, FieldType};
use crate::base::visitor::Visitor as FieldVisitor;
use crate::core::constraint::common::typed::Type;
use crate::core::constraint::string::enumeration::Enumeration;
use crate::core::constraint::string::max_length::MaxLength;
use crate::core::constraint::string::min_length::MinLength;
use crate::core::constraint::string::pattern::Pattern;
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

#[typetag::serde(name = "string")]
impl Field for StringField {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
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

    fn accept(&self, mut visitor: Box<dyn FieldVisitor>) {
        visitor.visit_string(self);
    }
}

#[cfg(test)]
mod tests {
    use crate::base::field::Field;
    use crate::core::constraint::string::enumeration::Enumeration;
    use crate::core::constraint::string::max_length::MaxLength;
    use crate::core::constraint::string::min_length::MinLength;
    use crate::core::constraint::string::pattern::Pattern;
    use crate::core::string::StringField;
    use regex::Regex;

    #[test]
    fn test_serialize() {
        let field = StringField {
            name: "body".to_string(),
            title: "Body".to_string(),
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
            r#"{"name":"body","title":"Body","enum":["meeting","email"],"maxLength":32,"minLength":8,"pattern":"[a-z]+"}"#
        );

        let dyn_field: &dyn Field = &field;
        let dyn_field_json = serde_json::to_string(&dyn_field).unwrap();
        assert_eq!(
            dyn_field_json,
            r#"{"type":"string","name":"body","title":"Body","enum":["meeting","email"],"maxLength":32,"minLength":8,"pattern":"[a-z]+"}"#
        );
    }

    #[test]
    fn test_deserialize() {
        let field_json = r#"
        {
            "type":"string",
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
