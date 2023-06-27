use std::fmt::{Display, Formatter};
use std::sync::Arc;
use crate::app::_services::domain::date_manipulation::{DateManipulationService, UtcInstant};
use super::AccountId;
use crate::app::_shared::domain::domain_error::{DomainError};
use crate::app::_services::domain::password_hashing::{HashedPassword, PasswordHashingService};

#[derive(Clone)]
pub struct Account {
    id: AccountId,
    name: String,
    email: String,
    password: HashedPassword,
    created_at: UtcInstant,
    updated_at: UtcInstant,
    date_manipulation: Arc<dyn DateManipulationService>,
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
                updated_at: self.date_manipulation.now(),
                date_manipulation: self.date_manipulation.clone(),
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
                updated_at: self.date_manipulation.now(),
                date_manipulation: self.date_manipulation.clone(),
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
                updated_at: self.date_manipulation.now(),
                date_manipulation: self.date_manipulation.clone(),
            }
        )
    }

    pub fn compare_password(&self, password: &str, hashing_service: &dyn PasswordHashingService) -> bool {
        self.password.compare(password, hashing_service)
    }

    pub fn id(&self) -> &AccountId {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn password(&self) -> &HashedPassword {
        &self.password
    }

    pub fn created_at(&self) -> &UtcInstant {
        &self.created_at
    }

    pub fn updated_at(&self) -> &UtcInstant {
        &self.updated_at
    }

    pub fn new(
        id: AccountId,
        name: String,
        email: String,
        password: HashedPassword,
        created_at: UtcInstant,
        updated_at: UtcInstant,
        date_manipulation_service: Arc<dyn DateManipulationService>,
    ) -> Result<Account, DomainError> {
        guards::guard_update_before_create(&updated_at, &created_at, date_manipulation_service.as_ref())
            .map(|_|
                Account {
                    id,
                    name,
                    email,
                    password,
                    created_at,
                    updated_at,
                    date_manipulation: date_manipulation_service,
                }
            )
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Account")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("password", &format!("{}", self.password))
            .field("email", &self.email)
            .field("updated_at", &self.updated_at)
            .field("created_at", &self.created_at)
            .finish()
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

mod guards {
    use crate::app::_services::domain::date_manipulation::{DateManipulationService, UtcInstant};
    use crate::app::_shared::domain::domain_error::DomainError;

    pub fn guard_update_before_create(
        updated_at: &UtcInstant,
        created_at: &UtcInstant,
        date_manipulation_service: &dyn DateManipulationService,
    ) -> Result<(), DomainError> {
        if date_manipulation_service.is_after(&updated_at, &created_at) {
            Err(DomainError::invalid_argument("Account cannot be updated before it is created".to_string()))
        } else {
            Ok(())
        }
    }
}
