use crate::domain::account::{Account, AccountId};
use crate::domain::shared::AggregateRepository;
use crate::domain::shared::domain_error::DomainError;

pub trait AccountRepository : AggregateRepository<Account> {
    fn find_by_id(&self, id: AccountId) -> Result<Account, DomainError>;

    fn is_email_already_used(&self, email: &str) -> Result<bool, DomainError>;
}
