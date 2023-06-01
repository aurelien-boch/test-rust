use crate::domain::services::password_hashing::{HashedPassword, PasswordHashingService};
use crate::domain::shared::domain_error::DomainError;

pub struct MockPasswordHashingService {}

impl PasswordHashingService for MockPasswordHashingService {
    fn hash_password(&self, password: &str) -> Result<HashedPassword, DomainError> {
        Ok(
            HashedPassword {
                value: password.to_string()
            }
        )
    }

    fn verify_password(&self, password: &str, hashed_password: &HashedPassword) -> bool {
        password == hashed_password.value
    }
}

impl MockPasswordHashingService {
    pub fn new() -> MockPasswordHashingService {
        MockPasswordHashingService {}
    }
}
