use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum DomainErrorCode {
    NotFound = 404,
    Conflict = 409,
    ServiceUnavailable = 502
}

#[derive(Debug)]
pub struct DomainError {
    pub message: String,
    pub error_type: DomainErrorCode,
}

impl Display for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DomainError: {}", self.message)
    }
}

impl Error for DomainError {}

impl DomainError {
    pub fn new(message: String, error_type: DomainErrorCode) -> DomainError {
        DomainError {
            message,
            error_type,
        }
    }
}
