use serde_json::Number;
use std::fmt;
use std::fmt::{Debug, Formatter};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("{message} ({constraint_name}")]
    VerifyFailed {
        message: String,
        constraint_name: String,
    },
    #[error("{message} ({constraint_name})")]
    VerificationFailed {
        message: String,
        constraint_name: String,
    },
}

pub type SchemaResult = Result<(), SchemaError>;

#[derive(Debug)]
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
    Object,
    Array,
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FieldType::String => write!(f, "string"),
            FieldType::Integer => write!(f, "integer"),
            FieldType::Float => write!(f, "float"),
            FieldType::Boolean => write!(f, "boolean"),
            FieldType::Array => write!(f, "array"),
            FieldType::Object => write!(f, "object"),
        }
    }
}

pub trait Field: Debug {
    fn name(&self) -> String;
    fn title(&self) -> String;
    fn get_type(&self) -> FieldType;
}

pub(crate) fn number_as_i64(value: &Number) -> Result<i64, SchemaError> {
    if value.is_i64() || value.is_u64() {
        match value.as_i64() {
            Some(v) => Ok(v),
            None => Err(SchemaError::VerificationFailed {
                message: format!("The value {} overflows integer", value),
                constraint_name: "Type".to_string(),
            }),
        }
    } else {
        Err(SchemaError::VerificationFailed {
            message: format!(
                "The value {} is {}, not {}",
                value,
                FieldType::Float,
                FieldType::Integer
            ),
            constraint_name: "Type".to_string(),
        })
    }
}
