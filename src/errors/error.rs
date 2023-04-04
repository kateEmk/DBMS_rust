use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    FileNotFound,
    CreateFileError(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::FileNotFound => write!(f, "File or directory not found"),
            ServiceError::CreateFileError(e) => write!(f, "Failed to create file: {}", e),
        }
    }
}

impl Error for ServiceError {}