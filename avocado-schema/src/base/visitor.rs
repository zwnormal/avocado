use crate::core::array::ArrayField;
use crate::core::boolean::BooleanField;
use crate::core::float::FloatField;
use crate::core::integer::IntegerField;
use crate::core::object::ObjectField;
use crate::core::string::StringField;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Field {
    Array(ArrayField),
    Boolean(BooleanField),
    Float(FloatField),
    Integer(IntegerField),
    Object(ObjectField),
    String(StringField),
}

impl Serialize for Field {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Field::Array(f) => f.serialize(serializer),
            Field::Boolean(f) => f.serialize(serializer),
            Field::Float(f) => f.serialize(serializer),
            Field::Integer(f) => f.serialize(serializer),
            Field::Object(f) => f.serialize(serializer),
            Field::String(f) => f.serialize(serializer),
        }
    }
}

pub trait Visitor: Debug {
    fn visit(&mut self, field: Arc<Field>);
}
