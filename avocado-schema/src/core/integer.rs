use crate::base::field::{Field, FieldType};
use crate::core::constraint::common::typed::Type;
use crate::core::constraint::integer::enumeration::Enumeration;
use crate::core::constraint::integer::exclusive_maximum::ExclusiveMaximum;
use crate::core::constraint::integer::exclusive_minimum::ExclusiveMinimum;
use crate::core::constraint::integer::maximum::Maximum;
use crate::core::constraint::integer::minimum::Minimum;
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "integer")]
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

impl Field for IntegerField {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        let mut constraints: Vec<Box<dyn Constraint>> = vec![Box::new(Type {
            typed: FieldType::Integer,
        })];
        if self.enumeration.is_some() {
            constraints.push(Box::new(self.enumeration.as_ref().unwrap().clone()))
        }
        if self.maximum.is_some() {
            constraints.push(Box::new(self.maximum.as_ref().unwrap().clone()))
        }
        if self.exclusive_maximum.is_some() {
            constraints.push(Box::new(self.exclusive_maximum.as_ref().unwrap().clone()))
        }
        if self.minimum.is_some() {
            constraints.push(Box::new(self.minimum.as_ref().unwrap().clone()))
        }
        if self.exclusive_minimum.is_some() {
            constraints.push(Box::new(self.exclusive_minimum.as_ref().unwrap().clone()))
        }
        constraints
    }
}
