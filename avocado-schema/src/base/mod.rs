use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub enum FieldType {
    String,
    Integer,
    Boolean,
    Object,
    Array,
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FieldType::String => write!(f, "string"),
            FieldType::Integer => write!(f, "integer"),
            FieldType::Boolean => write!(f, "boolean"),
            FieldType::Array => write!(f, "array"),
            FieldType::Object => write!(f, "object"),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Integer(i64),
    Boolean(bool),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Array(_) => write!(f, "Array(...)"),
            Value::Object(_) => write!(f, "Object(...)"),
            Value::String(v) => write!(f, "{}", v),
            Value::Boolean(v) => write!(f, "{}", v),
            Value::Integer(v) => write!(f, "{}", v),
        }
    }
}

pub trait Field: Debug {
    fn name(&self) -> String;
    fn title(&self) -> String;
    fn get_type(&self) -> FieldType;
}
