use crate::core::array::ArrayField;
use crate::core::boolean::BooleanField;
use crate::core::float::FloatField;
use crate::core::integer::IntegerField;
use crate::core::object::ObjectField;
use crate::core::string::StringField;
use std::fmt::Debug;

pub trait Visitor: Debug {
    fn visit_array(&mut self, array: &ArrayField);
    fn visit_boolean(&mut self, boolean: &BooleanField);
    fn visit_float(&mut self, float: &FloatField);
    fn visit_integer(&mut self, integer: &IntegerField);
    fn visit_object(&mut self, object: &ObjectField);
    fn visit_string(&mut self, string: &StringField);
}
