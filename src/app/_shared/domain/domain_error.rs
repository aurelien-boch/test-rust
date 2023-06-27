use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorType {
    InvalidArgument = 400,
    NotFound = 404,
    AlreadyExists = 409,
    Unknown = 500,
    ServiceUnavailable = 503,
}

pub struct DomainError {
    message: String,
    error_type: ErrorType,
}

impl DomainError {
    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn error_type(&self) -> &ErrorType {
        &self.error_type
    }

    pub fn invalid_argument(message: String) -> DomainError {
        DomainError {
            message,
            error_type: ErrorType::InvalidArgument,
        }
    }

    pub fn not_found(message: String) -> DomainError {
        DomainError {
            message,
            error_type: ErrorType::NotFound,
        }
    }

    pub fn already_exists(message: String) -> DomainError {
        DomainError {
            message,
            error_type: ErrorType::AlreadyExists,
        }
    }

    pub fn unknown(message: String) -> DomainError {
        DomainError {
            message,
            error_type: ErrorType::Unknown,
        }
    }

    pub fn service_unavailable(message: String) -> DomainError {
        DomainError {
            message,
            error_type: ErrorType::ServiceUnavailable,
        }
    }
}

impl Display for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DomainError")
            .field("message", &self.message)
            .field("error_type", &self.error_type)
            .finish()
    }
}

impl Debug for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self).as_str())
    }
}

impl Error for DomainError {}
