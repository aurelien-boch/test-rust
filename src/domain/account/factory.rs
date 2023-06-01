use std::sync::Arc;
use std::time::{Instant};
use crate::domain::account::account_id::AccountId;
use crate::domain::account::aggregate::Account;
use crate::domain::services::id_generation::IdGenerationService;
use crate::domain::services::password_hashing::{PasswordHashingService};
use crate::domain::shared::domain_error::{DomainError};

pub struct AccountFactory {
    password_hashing_service: Arc<dyn PasswordHashingService>,
    id_generation_service: Arc<dyn IdGenerationService>,
}

impl AccountFactory {
    pub fn new(
        password_hashing_service: Arc<dyn PasswordHashingService>,
        id_generation_service: Arc<dyn IdGenerationService>,
    ) -> AccountFactory {
        AccountFactory {
            password_hashing_service,
            id_generation_service
        }
    }

    pub fn submit_account(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<Account, DomainError> {
        let id = self.id_generation_service.generate();
        let now = Instant::now();

        self.password_hashing_service.hash_password(&password)
            .map(|password| Account {
                id: AccountId::from(id),
                name,
                email,
                password,
                created_at: now,
                updated_at: now,
            })
    }
}
