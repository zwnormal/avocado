use crate::base::SchemaError;
use serde_json::Value;
use std::collections::HashMap;

pub struct Validator {
    pub value: Value,
    pub field_names: Vec<String>,
    pub errors: HashMap<String, Vec<SchemaError>>,
}
