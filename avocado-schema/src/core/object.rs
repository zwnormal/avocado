use crate::base::field::{Field, FieldType};
use crate::base::visitor::FieldEnum;
use crate::core::constraint::common::typed::Type;
use crate::core::constraint::object::required::Required;
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "object")]
pub struct ObjectField {
    pub name: String,
    pub title: String,
    pub properties: HashMap<String, Arc<FieldEnum>>,
    pub required: Option<Required>,
}

impl Field for ObjectField {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn get_type(&self) -> FieldType {
        FieldType::Object
    }

    fn into_enum(self) -> FieldEnum {
        FieldEnum::Object(self)
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        let mut constraints: Vec<Box<dyn Constraint>> = vec![Box::new(Type {
            typed: FieldType::Object,
        })];
        if let Some(c) = &self.required {
            constraints.push(Box::new(c.clone()))
        }
        constraints
    }
}

#[derive(Default)]
pub struct ObjectFieldBuilder {
    name: String,
    title: String,
    properties: HashMap<String, Arc<FieldEnum>>,
    required: Option<Vec<String>>,
}

impl ObjectFieldBuilder {
    pub fn new() -> Self {
        ObjectFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn property(mut self, name: &'static str, field: impl Field) -> Self {
        self.properties
            .insert(name.to_string(), Arc::new(field.into_enum()));
        self
    }

    pub fn required(mut self, names: Vec<String>) -> Self {
        self.required = Some(names);
        self
    }

    pub fn build(self) -> ObjectField {
        ObjectField {
            name: self.name,
            title: self.title,
            properties: self.properties,
            required: self.required.map(|required| Required { required }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::object::ObjectField;

    #[test]
    fn test_deserialize() {
        let valid_schema_json = r#"
        {
            "type": "object",
            "name": "client",
            "title": "Client",
            "properties": {
                "first_name": {
                    "type": "string",
                    "name": "first_name",
                    "title": "First Name",
                    "maxLength": 32,
                    "minLength": 8
                },
                "last_name": {
                    "type": "string",
                    "name": "last_name",
                    "title": "Last Name",
                    "maxLength": 32,
                    "minLength": 8,
                    "pattern": "[a-zA-Z]+"
                }
            }
        }"#;
        assert!(serde_json::from_str::<ObjectField>(valid_schema_json).is_ok());

        let invalid_schema_json = r#"
        {
            "type": "object",
            "name": "client",
            "title": "Client",
            "properties": {
                "first_name": {
                    "type": "string",
                    "name": "first_name",
                    "title": "First Name",
                    "maxLength": 32,
                    "minLength": -1,
                },
                "last_name": {
                    "type": "string",
                    "name": "last_name",
                    "title": "Last Name",
                    "maxLength": 32,
                    "minLength": 8
                }
            }
        }"#;
        assert!(serde_json::from_str::<ObjectField>(invalid_schema_json).is_err());
    }
}
