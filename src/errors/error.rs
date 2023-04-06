use thiserror::Error;
use std::fmt;
use crate::prelude::{OperationFailure, TableFailure};

#[derive(Debug)]
pub enum ServiceError {
    FileNotFound,
    CreateFileError(String),
    TypeDoesntMatch,
    ErrorAddingToTheFile
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
}
pub type HandlerResult<T = ()> = Result<T, HandlerError>;
