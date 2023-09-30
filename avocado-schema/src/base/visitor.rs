use crate::core::array::ArrayField;
use crate::core::boolean::BooleanField;
use crate::core::float::FloatField;
use crate::core::integer::IntegerField;
use crate::core::object::ObjectField;
use crate::core::string::StringField;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FieldEnum {
    Array(ArrayField),
    Boolean(BooleanField),
    Float(FloatField),
    Integer(IntegerField),
    Object(ObjectField),
    String(StringField),
}

pub trait Visitor: Debug {
    fn visit(&mut self, field: Arc<FieldEnum>);
}
