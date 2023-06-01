use std::fmt::{Display, Formatter};
use std::time::Instant;
use crate::domain::account::account_id::AccountId;
use crate::domain::services::password_hashing::{HashedPassword, PasswordHashingService};
use crate::domain::shared::domain_error::DomainError;

#[derive(Clone)]
pub struct Account {
    pub id: AccountId,
    pub name: String,
    pub email: String,
    pub password: HashedPassword,
    pub created_at: Instant,
    pub updated_at: Instant,
}

impl Account {
    pub fn update_name(&self, name: String) -> Result<Account, DomainError> {
        Ok(
            Account {
                id: self.id.clone(),
                name,
                email: self.email.clone(),
                password: self.password.clone(),
                created_at: self.created_at,
                updated_at: Instant::now(),
            }
        )
    }

    pub fn update_email(&self, email: String) -> Result<Account, DomainError> {
        Ok(
            Account {
                id: self.id.clone(),
                name: self.name.clone(),
                email,
                password: self.password.clone(),
                created_at: self.created_at,
                updated_at: Instant::now(),
            }
        )
    }

    pub fn update_password(&self, password: HashedPassword) -> Result<Account, DomainError> {
        Ok(
            Account {
                id: self.id.clone(),
                name: self.name.clone(),
                email: self.email.clone(),
                password,
                created_at: self.created_at,
                updated_at: Instant::now(),
            }
        )
    }

    pub fn compare_password(&self, password: &str, hashing_service: &dyn PasswordHashingService) -> bool {
        self.password.compare(password, hashing_service)
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Account")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("email", &self.email)
            .field("updated_at", &self.updated_at)
            .field("created_at", &self.created_at)
            .finish()
    }
}
