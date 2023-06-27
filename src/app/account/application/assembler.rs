use std::sync::Arc;
use crate::app::_services::domain::date_manipulation::{DateManipulationAssembler, DateManipulationService};
use crate::app::_services::domain::password_hashing::PasswordHashingAssembler;
use crate::app::_shared::domain::domain_error::DomainError;
use crate::app::account::application::dto::{AccountDto, AccountIdDto};
use crate::app::account::domain::{Account, AccountId};

pub struct AccountAssembler {
    password_hashing_assembler: Arc<PasswordHashingAssembler>,
    date_manipulation_assembler: Arc<DateManipulationAssembler>,
    date_manipulation_service: Arc<dyn DateManipulationService>,
}

impl AccountAssembler {
    pub fn new(
        password_hashing_assembler: Arc<PasswordHashingAssembler>,
        date_manipulation_assembler: Arc<DateManipulationAssembler>,
        date_manipulation_service: Arc<dyn DateManipulationService>,
    ) -> AccountAssembler {
        AccountAssembler {
            password_hashing_assembler,
            date_manipulation_assembler,
            date_manipulation_service,
        }
    }

    pub fn dehydrate(&self, account: &Account) -> AccountDto {
        AccountDto {
            id: self.dehydrate_account_id(account.id()),
            name: account.name().clone(),
            email: account.email().clone(),
            password: self.password_hashing_assembler.dehydrate(account.password()),
            created_at: self.date_manipulation_assembler.dehydrate(account.created_at()),
            updated_at: self.date_manipulation_assembler.dehydrate(account.updated_at()),
        }
    }

    pub fn hydrate(
        &self,
        dto: &AccountDto
    ) -> Result<Account, DomainError> {
        Account::new(
            self.hydrate_account_id(&dto.id),
            dto.name.clone(),
            dto.email.clone(),
            self.password_hashing_assembler.hydrate(&dto.password),
            self.date_manipulation_assembler.hydrate(&dto.created_at)?,
            self.date_manipulation_assembler.hydrate(&dto.updated_at)?,
            self.date_manipulation_service.clone(),
        )
    }

    pub fn dehydrate_account_id(&self, id: &AccountId) -> AccountIdDto {
        AccountIdDto {
            value: id.value().to_string(),
        }
    }

    pub fn hydrate_account_id(&self, dto: &AccountIdDto) -> AccountId {
        AccountId::new(dto.value.clone())
    }
}
