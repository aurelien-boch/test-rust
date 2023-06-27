use std::sync::Arc;
use crate::app::_services::domain::date_manipulation::DateManipulationService;
use crate::app::_shared::domain::domain_error::DomainError;
use crate::app::_services::domain::id_generation::IdGenerationService;
use crate::app::_services::domain::password_hashing::PasswordHashingService;
use super::AccountId;
use super::Account;

pub struct AccountFactory {
    password_hashing_service: Arc<dyn PasswordHashingService>,
    id_generation_service: Arc<dyn IdGenerationService>,
    date_manipulation_service: Arc<dyn DateManipulationService>,
}

impl AccountFactory {
    pub fn new(
        password_hashing_service: Arc<dyn PasswordHashingService>,
        id_generation_service: Arc<dyn IdGenerationService>,
        date_manipulation_service: Arc<dyn DateManipulationService>,
    ) -> AccountFactory {
        AccountFactory {
            password_hashing_service,
            id_generation_service,
            date_manipulation_service,
        }
    }

    pub fn submit_account(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<Account, DomainError> {
        let id = self.id_generation_service.generate();
        let now = self.date_manipulation_service.now();

        self.password_hashing_service.hash_password(&password)
            .and_then(|password| Account::new(
                AccountId::from(id),
                name,
                email,
                password,
                now,
                now,
                self.date_manipulation_service.clone(),
            ))
    }
}
