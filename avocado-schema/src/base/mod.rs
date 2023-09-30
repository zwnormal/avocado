use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("{message} ({constraint_name})")]
    Validation {
        message: String,
        constraint_name: String,
    },
}

pub type SchemaResult = Result<(), SchemaError>;

pub(crate) mod field;
pub(crate) mod visitor;
