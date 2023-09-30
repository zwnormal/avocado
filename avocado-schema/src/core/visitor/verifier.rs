use crate::base::field::Field;
use crate::base::visitor::Visitor;
use crate::base::SchemaError;
use crate::core::array::ArrayField;
use crate::core::boolean::BooleanField;
use crate::core::float::FloatField;
use crate::core::integer::IntegerField;
use crate::core::object::ObjectField;
use crate::core::string::StringField;
use std::collections::HashMap;

/// Verify whether a schema definition is valid
#[derive(Debug)]
pub struct Verifier {
    pub field_names: Vec<String>,
    pub errors: HashMap<String, Vec<SchemaError>>,
}

impl Verifier {
    pub fn new() -> Self {
        Self {
            field_names: vec![],
            errors: HashMap::new(),
        }
    }

    fn verify(&mut self, field: &(impl Field + ?Sized)) {
        for constraint in field.constrains() {
            match constraint.verify() {
                Ok(_) => {}
                Err(e) => {
                    let field = self.field_names.clone().join("/");
                    if self.errors.contains_key(field.as_str()) {
                        self.errors.get_mut(field.as_str()).unwrap().push(e);
                    } else {
                        self.errors.insert(field, vec![e]);
                    }
                }
            }
        }
    }
}

impl Visitor for Verifier {
    fn visit_array(&mut self, array: &ArrayField) {
        self.field_names.push(array.name.clone());
        self.verify(&*array.item);
        self.field_names.pop();
    }

    fn visit_boolean(&mut self, boolean: &BooleanField) {
        self.field_names.push(boolean.name.clone());
        self.verify(boolean);
        self.field_names.pop();
    }

    fn visit_float(&mut self, float: &FloatField) {
        self.field_names.push(float.name.clone());
        self.verify(float);
        self.field_names.pop();
    }

    fn visit_integer(&mut self, integer: &IntegerField) {
        self.field_names.push(integer.name.clone());
        self.verify(integer);
        self.field_names.pop();
    }

    fn visit_object(&mut self, object: &ObjectField) {
        self.field_names.push(object.name.clone());
        for (name, field) in &object.properties {
            self.field_names.push(name.clone());
            self.verify(&**field);
            self.field_names.pop();
        }
        self.field_names.pop();
    }

    fn visit_string(&mut self, string: &StringField) {
        self.field_names.push(string.name.clone());
        self.verify(string);
        self.field_names.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::base::visitor::Visitor;
    use crate::core::object::ObjectField;
    use crate::core::visitor::verifier::Verifier;

    #[test]
    fn test_verify() {
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
                    "minLength": 8                    
                }
            }
        }"#;
        let valid_schema: ObjectField = serde_json::from_str(valid_schema_json).unwrap();
        let mut verifier = Verifier::new();
        verifier.visit_object(&valid_schema);
        assert!(verifier.errors.is_empty());

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
                    "maxLength": 0,
                    "minLength": 8
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
        let invalid_schema: ObjectField = serde_json::from_str(invalid_schema_json).unwrap();
        let mut verifier = Verifier::new();
        verifier.visit_object(&invalid_schema);
        print!("{:?}", verifier.errors);
        assert!(verifier
            .errors
            .get("client/first_name")
            .unwrap()
            .get(0)
            .unwrap()
            .to_string()
            .contains("The max length is 0"))
    }
}
