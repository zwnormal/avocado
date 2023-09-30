pub mod base;
pub mod core;

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
                    "minLength": 8                    
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
