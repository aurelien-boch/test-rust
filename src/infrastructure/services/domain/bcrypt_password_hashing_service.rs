use crate::domain::services::password_hashing::{HashedPassword, PasswordHashingService};
use crate::domain::shared::domain_error::{DomainError, DomainErrorCode};

pub struct BcryptPasswordHashingService {}

impl BcryptPasswordHashingService {
    pub fn new() -> BcryptPasswordHashingService {
        BcryptPasswordHashingService {}
    }
}

impl PasswordHashingService for BcryptPasswordHashingService {
    fn hash_password(&self, password: &str) -> Result<HashedPassword, DomainError> {
        bcrypt::hash(password, 4)
            .map(HashedPassword::new)
            .map_err(|err| DomainError::new(
                format!("Error hashing password: {}", err),
                DomainErrorCode::ServiceUnavailable
            ))
    }

    fn verify_password(&self, password: &str, hashed_password: &HashedPassword) -> bool {
        bcrypt::verify(password, &hashed_password.value).unwrap_or(false)
    }
}
