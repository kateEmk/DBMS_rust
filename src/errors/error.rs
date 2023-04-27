use thiserror::Error;
use std::fmt;
use crate::prelude::{OperationFailure, TableFailure};

#[derive(Debug)]
pub enum ServiceError {
    FileNotFound,
    CreateFileError(String),
    TypeDoesntMatch,
    ErrorAddingToTheFile,
    TooManyArgs,
    RecordDoesntExist,
    RowDoesntExist
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::FileNotFound => write!(f, "File or directory not found"),
            ServiceError::CreateFileError(e) => write!(f, "Failed to create file: {}", e),
            ServiceError::TypeDoesntMatch => write!(f, "Type of field value doesn't match with \
            type of the column."),
            ServiceError::ErrorAddingToTheFile => write!(f, "Error while adding record to the file\
            ."),
            ServiceError::TooManyArgs => write!(f, "Too many arguments were given."),
            ServiceError::RecordDoesntExist => write!(f, "Record in this table doesn't exist."),
            ServiceError::RowDoesntExist => write!(f, "This row doesn't seem to be exist in the \
            table\
            ."),
        }
    }
}

impl std::error::Error for ServiceError {}

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("{0}")]
    TableError(#[from] TableFailure),
    #[error("{0}")]
    OperationError(#[from] OperationFailure),
    #[error("{0}")]
    ServiceErrors(#[from] ServiceError),
}
pub type HandlerResult<T = ()> = Result<T, HandlerError>;
