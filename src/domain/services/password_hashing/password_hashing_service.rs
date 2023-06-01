use crate::domain::services::password_hashing::HashedPassword;
use crate::domain::shared::domain_error::DomainError;

pub trait PasswordHashingService {
    fn hash_password(&self, password: &str) -> Result<HashedPassword, DomainError>;
    fn verify_password(&self, password: &str, hashed_password: &HashedPassword) -> bool;
}
