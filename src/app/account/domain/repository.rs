use crate::app::_shared::domain::aggregate_repository::AggregateRepository;
use crate::app::_shared::domain::domain_error::DomainError;
use crate::app::account::application::dto::{AccountDto, AccountIdDto};

pub trait AccountRepository : AggregateRepository<AccountDto> {
    fn find_by_id(&mut self, id: AccountIdDto) -> Result<Option<AccountDto>, DomainError>;

    fn is_email_already_used(&mut self, email: &str) -> Result<bool, DomainError>;
}
