use crate::app::_shared::domain::domain_error::DomainError;
use super::HashedPassword;

pub trait PasswordHashingService {
    fn hash_password(&self, password: &str) -> Result<HashedPassword, DomainError>;
    fn verify_password(&self, password: &str, hashed_password: &HashedPassword) -> bool;
}
