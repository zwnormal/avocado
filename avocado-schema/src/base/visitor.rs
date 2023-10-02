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
pub enum FieldEnum {
    Array(ArrayField),
    Boolean(BooleanField),
    Float(FloatField),
    Integer(IntegerField),
    Object(ObjectField),
    String(StringField),
}

impl Serialize for FieldEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            FieldEnum::Array(f) => f.serialize(serializer),
            FieldEnum::Boolean(f) => f.serialize(serializer),
            FieldEnum::Float(f) => f.serialize(serializer),
            FieldEnum::Integer(f) => f.serialize(serializer),
            FieldEnum::Object(f) => f.serialize(serializer),
            FieldEnum::String(f) => f.serialize(serializer),
        }
    }
}

pub trait Visitor: Debug {
    fn visit(&mut self, field: Arc<FieldEnum>);
}
