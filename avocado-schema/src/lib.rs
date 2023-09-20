pub mod base;
pub mod core;

#[cfg(test)]
mod tests {
    use crate::base::Value;
    use std::collections::HashMap;

    struct Document {
        subject: String,
        body: String,
    }

    impl From<Document> for Value {
        fn from(value: Document) -> Self {
            Value::Object(HashMap::from([
                ("subject".to_string(), Value::String(value.subject)),
                ("bode".to_string(), Value::String(value.body)),
            ]))
        }
    }
}
