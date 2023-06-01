use crate::application::shared::UseCaseContext;
use crate::domain::account::AccountId;
use crate::domain::shared::domain_error::DomainError;

pub struct Params {
    pub id: AccountId,
    pub new_name: String
}

pub fn handler(context: &mut UseCaseContext, params: Params) -> Result<(), DomainError> {
    context.repositories.account_repository.find_by_id(params.id)
        .and_then(|account| account.update_name(params.new_name))
        .and_then(|account| context.repositories.account_repository.save(account))
}
