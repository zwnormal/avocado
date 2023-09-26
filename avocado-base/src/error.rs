use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

pub fn get_root_message(mut err: &dyn Error) -> String {
    while let Some(source) = err.source() {
        err = source;
    }
    err.to_string()
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ValidationMessageKind {
    Struct(HashMap<String, ValidationMessageKind>),
    List(Vec<HashMap<String, ValidationMessageKind>>),
    Field(Vec<String>),
}

#[derive(Debug, Serialize)]
pub struct ValidationMessages {
    pub messages: HashMap<String, ValidationMessageKind>,
}

impl From<ValidationErrors> for ValidationMessages {
    fn from(errors: ValidationErrors) -> Self {
        Self {
            messages: ValidationErrorsParser.parse(errors),
        }
    }
}

pub struct ValidationErrorsParser;

impl ValidationErrorsParser {
    pub fn parse(&self, errors: ValidationErrors) -> HashMap<String, ValidationMessageKind> {
        self.visit_validation_errors(errors)
    }

    fn visit_validation_errors(
        &self,
        errors: ValidationErrors,
    ) -> HashMap<String, ValidationMessageKind> {
        let mut result = HashMap::new();
        for (field, error) in errors.into_errors() {
            match error {
                ValidationErrorsKind::Struct(e) => {
                    result.insert(
                        field.to_string(),
                        ValidationMessageKind::Struct(self.visit_validation_errors(*e)),
                    );
                }
                ValidationErrorsKind::List(e) => {
                    let mut child_errors = vec![];
                    for (_, e) in e {
                        child_errors.push(self.visit_validation_errors(*e));
                    }
                    result.insert(field.to_string(), ValidationMessageKind::List(child_errors));
                }
                ValidationErrorsKind::Field(e) => {
                    result.insert(
                        field.to_string(),
                        ValidationMessageKind::Field(
                            e.into_iter()
                                .map(|ve| self.visit_validation_error(ve))
                                .collect(),
                        ),
                    );
                }
            }
        }
        result
    }

    fn visit_validation_error(&self, error: ValidationError) -> String {
        match error.message {
            None => "".to_string(),
            Some(m) => m.to_string(),
        }
    }
}
