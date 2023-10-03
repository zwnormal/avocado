use crate::base::field::{Field, FieldType};
use crate::base::visitor::FieldEnum;
use crate::core::constraint::common::typed::Type;
use crate::core::constraint::float::enumeration::Enumeration;
use crate::core::constraint::float::exclusive_maximum::ExclusiveMaximum;
use crate::core::constraint::float::exclusive_minimum::ExclusiveMinimum;
use crate::core::constraint::float::maximum::Maximum;
use crate::core::constraint::float::minimum::Minimum;
use crate::core::constraint::Constraint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "float")]
pub struct FloatField {
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

impl Field for FloatField {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn get_type(&self) -> FieldType {
        FieldType::Float
    }

    fn into_enum(self) -> FieldEnum {
        FieldEnum::Float(self)
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        let mut constraints: Vec<Box<dyn Constraint>> = vec![Box::new(Type {
            typed: FieldType::Float,
        })];
        if let Some(c) = &self.enumeration {
            constraints.push(Box::new(c.clone()))
        }
        if let Some(c) = &self.maximum {
            constraints.push(Box::new(c.clone()))
        }
        if let Some(c) = &self.exclusive_maximum {
            constraints.push(Box::new(c.clone()))
        }
        if let Some(c) = &self.minimum {
            constraints.push(Box::new(c.clone()))
        }
        if let Some(c) = &self.exclusive_minimum {
            constraints.push(Box::new(c.clone()))
        }
        constraints
    }
}

#[derive(Default)]
pub struct FloatFieldBuilder {
    name: String,
    title: String,
    enumeration: Option<Vec<f64>>,
    maximum: Option<f64>,
    exclusive_maximum: Option<f64>,
    minimum: Option<f64>,
    exclusive_minimum: Option<f64>,
}

impl FloatFieldBuilder {
    pub fn new() -> Self {
        FloatFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn enumeration(mut self, numbers: Vec<f64>) -> Self {
        self.enumeration = Some(numbers);
        self
    }

    pub fn maximum(mut self, max: f64) -> Self {
        self.maximum = Some(max);
        self
    }

    pub fn exclusive_maximum(mut self, max: f64) -> Self {
        self.exclusive_maximum = Some(max);
        self
    }

    pub fn minimum(mut self, min: f64) -> Self {
        self.minimum = Some(min);
        self
    }

    pub fn exclusive_minimum(mut self, min: f64) -> Self {
        self.exclusive_minimum = Some(min);
        self
    }

    pub fn build(self) -> FloatField {
        FloatField {
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
