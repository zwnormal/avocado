use crate::core::constraint::Constraint;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
    Object,
    Array,
}

impl Serialize for FieldType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{}", self).as_str())
    }
}

impl<'de> Deserialize<'de> for FieldType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FieldTypeVisitor)
    }
}

struct FieldTypeVisitor;

impl<'de> Visitor<'de> for FieldTypeVisitor {
    type Value = FieldType;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "field type needs to be a valid type of string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match v {
            "string" => Ok(FieldType::String),
            "integer" => Ok(FieldType::Integer),
            "float" => Ok(FieldType::Float),
            "boolean" => Ok(FieldType::Boolean),
            "array" => Ok(FieldType::Array),
            "object" => Ok(FieldType::Object),
            _ => Err(Error::custom("invalid field type")),
        }
    }
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
    fn constrains(&self) -> Vec<Box<dyn Constraint>>;
}
