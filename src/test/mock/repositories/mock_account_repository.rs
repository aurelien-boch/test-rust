use crate::domain::account::{Account, AccountId, AccountRepository};
use crate::domain::shared::AggregateRepository;
use crate::domain::shared::domain_error::{DomainError, DomainErrorCode};

pub struct MockAccountRepository {
    accounts: Vec<Account>,
}

impl AggregateRepository<Account> for MockAccountRepository {
    fn save(&mut self, aggregate: Account) -> Result<(), DomainError> {
        self
            .accounts
            .iter()
            .position(|account| account.id == aggregate.id)
            .map(|index| self.accounts[index] = aggregate.clone())
            .or_else(|| {
                self.accounts.push(aggregate.clone());
                Some(())
            })
            .ok_or_else(|| DomainError::new(
                format!("Error while saving user. User: {}", aggregate),
                DomainErrorCode::ServiceUnavailable,
            ))
    }

    fn delete(&mut self, aggregate: Account) -> Result<(), DomainError> {
        self
            .accounts
            .retain(|account| account.id != aggregate.id);

        Ok(())
    }
}

impl AccountRepository for MockAccountRepository {
    fn find_by_id(&self, id: AccountId) -> Result<Account, DomainError> {
        self.accounts
            .iter()
            .find(|account| account.id == id)
            .cloned()
            .ok_or_else(|| DomainError::new(
                format!("Error while finding user by id. Id: {}", id),
                DomainErrorCode::NotFound,
            ))
    }

    fn is_email_already_used(&self, email: &str) -> Result<bool, DomainError> {
        Ok(
            self.accounts
                .iter()
                .any(|account| account.email == email)
        )
    }
}

impl MockAccountRepository {
    pub fn new() -> MockAccountRepository {
        MockAccountRepository {
            accounts: Vec::new()
        }
    }
}
