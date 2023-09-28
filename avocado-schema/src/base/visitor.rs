use crate::core::array::ArrayField;
use crate::core::boolean::BooleanField;
use crate::core::float::FloatField;
use crate::core::integer::IntegerField;
use crate::core::object::ObjectField;
use crate::core::string::StringField;
use std::fmt::Debug;

pub trait Visitor: Debug {
    fn visit_array(&self, array: &ArrayField);
    fn visit_boolean(&self, boolean: &BooleanField);
    fn visit_float(&self, float: &FloatField);
    fn visit_integer(&self, integer: &IntegerField);
    fn visit_object(&self, object: &ObjectField);
    fn visit_string(&self, string: &StringField);
}
