use crate::base::field::{Field, FieldType};
use crate::base::visitor::FieldEnum;
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

    fn get_type(&self) -> FieldType {
        FieldType::Integer
    }

    fn into_enum(self) -> FieldEnum {
        FieldEnum::Integer(self)
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

#[derive(Default)]
pub struct IntegerFieldBuilder {
    name: String,
    title: String,
    enumeration: Option<Vec<i64>>,
    maximum: Option<i64>,
    exclusive_maximum: Option<i64>,
    minimum: Option<i64>,
    exclusive_minimum: Option<i64>,
}

impl IntegerFieldBuilder {
    pub fn new() -> Self {
        IntegerFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn enumeration(mut self, numbers: Vec<i64>) -> Self {
        self.enumeration = Some(numbers);
        self
    }

    pub fn maximum(mut self, max: i64) -> Self {
        self.maximum = Some(max);
        self
    }

    pub fn exclusive_maximum(mut self, max: i64) -> Self {
        self.exclusive_maximum = Some(max);
        self
    }

    pub fn minimum(mut self, min: i64) -> Self {
        self.minimum = Some(min);
        self
    }

    pub fn exclusive_minimum(mut self, min: i64) -> Self {
        self.exclusive_minimum = Some(min);
        self
    }

    pub fn build(self) -> IntegerField {
        IntegerField {
            name: self.name,
            title: self.title,
            enumeration: self.enumeration.map(|values| Enumeration { values }),
            maximum: self.maximum.map(|max_val| Maximum { max_val }),
            exclusive_maximum: self
                .exclusive_maximum
                .map(|max_val| ExclusiveMaximum { max_val }),
            minimum: self.minimum.map(|min_val| Minimum { min_val }),
            exclusive_minimum: self
                .exclusive_minimum
                .map(|min_val| ExclusiveMinimum { min_val }),
        }
    }
}
