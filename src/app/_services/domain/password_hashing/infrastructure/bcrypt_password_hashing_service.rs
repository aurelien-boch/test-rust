use crate::app::_shared::domain::domain_error::{DomainError};
use crate::app::_services::domain::password_hashing::HashedPassword;
use crate::app::_services::domain::password_hashing::PasswordHashingService;

pub struct BcryptPasswordHashingService {}

impl PasswordHashingService for BcryptPasswordHashingService {
    fn hash_password(&self, password: &str) -> Result<HashedPassword, DomainError> {
        bcrypt::hash(password, 4)
            .map(HashedPassword::new)
            .map_err(|err| DomainError::service_unavailable(format!("Error hashing password: {}", err)))
    }

    fn verify_password(&self, password: &str, hashed_password: &HashedPassword) -> bool {
        bcrypt::verify(password, &hashed_password.value).unwrap_or(false)
    }
}
