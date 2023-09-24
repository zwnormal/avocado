use crate::base::{Field, FieldType};
use crate::core::constraint::integer::enumeration::Enumeration;
use crate::core::constraint::integer::exclusive_maximum::ExclusiveMaximum;
use crate::core::constraint::integer::exclusive_minimum::ExclusiveMinimum;
use crate::core::constraint::integer::maximum::Maximum;
use crate::core::constraint::integer::minimum::Minimum;

#[derive(Debug)]
pub struct IntegerField {
    pub name: String,
    pub title: String,
    pub enumeration: Option<Enumeration>,
    pub maximum: Option<Maximum>,
    pub exclusive_maximum: Option<ExclusiveMaximum>,
    pub minimum: Option<Minimum>,
    pub exclusive_minimum: Option<ExclusiveMinimum>,
}

impl IntegerField {
    const TYPE: FieldType = FieldType::Integer;
}

impl Field for IntegerField {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn get_type(&self) -> FieldType {
        Self::TYPE
    }
}
