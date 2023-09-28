use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("{message} ({constraint_name}")]
    Verify {
        message: String,
        constraint_name: String,
    },
    #[error("{message} ({constraint_name})")]
    Verification {
        message: String,
        constraint_name: String,
    },
}

pub type SchemaResult = Result<(), SchemaError>;

pub(crate) mod field;
