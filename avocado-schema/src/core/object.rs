use crate::base::field::{Field, FieldType};
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
    pub properties: HashMap<String, Arc<crate::base::visitor::Field>>,
    pub required: Option<Required>,
}

impl Field for ObjectField {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        let mut constraints: Vec<Box<dyn Constraint>> = vec![Box::new(Type {
            typed: FieldType::Object,
        })];
        if self.required.is_some() {
            constraints.push(Box::new(self.required.as_ref().unwrap().clone()))
        }
        constraints
    }
}

#[cfg(test)]
mod tests {
    use crate::base::visitor::Field;
    use crate::core::constraint::object::required::Required;
    use crate::core::constraint::string::max_length::MaxLength;
    use crate::core::constraint::string::min_length::MinLength;
    use crate::core::constraint::string::pattern::Pattern;
    use crate::core::object::ObjectField;
    use crate::core::string::StringField;
    use regex::Regex;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn test_serialize() {
        let field = ObjectField {
            name: "client".to_string(),
            title: "Client".to_string(),
            properties: HashMap::from([(
                "first_name".to_string(),
                Arc::new(Field::String(StringField {
                    name: "first_name".to_string(),
                    title: "First Name".to_string(),
                    enumeration: None,
                    max_length: Some(MaxLength { max_length: 32 }),
                    min_length: Some(MinLength { min_length: 4 }),
                    pattern: Some(Pattern {
                        pattern: Regex::new(r"[a-z]+").unwrap(),
                    }),
                })),
            )]),
            required: Some(Required { required: vec![] }),
        };
        print!("{}", serde_json::to_string(&field).unwrap());
    }
}
