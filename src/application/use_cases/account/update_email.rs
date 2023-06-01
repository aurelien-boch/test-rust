use crate::application::shared::UseCaseContext;
use crate::domain::account::AccountId;
use crate::domain::shared::domain_error::DomainError;

pub struct Params {
    pub id: AccountId,
    pub new_email: String
}


pub fn handler(context: &mut UseCaseContext, params: Params) -> Result<(), DomainError> {
    context.repositories.account_repository.find_by_id(params.id)
        .and_then(|account| account.update_email(params.new_email))
        .and_then(|account| context.repositories.account_repository.save(account))
}
