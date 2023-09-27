use crate::base::Field;
use crate::core::constraint::integer::enumeration::Enumeration;
use crate::core::constraint::integer::exclusive_maximum::ExclusiveMaximum;
use crate::core::constraint::integer::exclusive_minimum::ExclusiveMinimum;
use crate::core::constraint::integer::maximum::Maximum;
use crate::core::constraint::integer::minimum::Minimum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IntegerField {
    pub name: String,
    pub title: String,
    #[serde(flatten)]
    pub enumeration: Option<Enumeration>,
    #[serde(flatten)]
    pub maximum: Option<Maximum>,
    #[serde(flatten)]
    pub exclusive_maximum: Option<ExclusiveMaximum>,
    #[serde(flatten)]
    pub minimum: Option<Minimum>,
    #[serde(flatten)]
    pub exclusive_minimum: Option<ExclusiveMinimum>,
}

#[typetag::serde(name = "integer")]
impl Field for IntegerField {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }
}
