use crate::application::shared::UseCaseContext;
use crate::domain::shared::domain_error::DomainError;
use crate::domain::account::{AccountId};

pub struct Params {
    pub id: AccountId,
}


pub fn handler(context: &mut UseCaseContext, params: Params) -> Result<(), DomainError> {
    context.repositories.account_repository.find_by_id(params.id)
        .and_then(|account| context.repositories.account_repository.delete(account))
}
