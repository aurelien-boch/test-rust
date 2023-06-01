use crate::application::shared::UseCaseContext;
use crate::domain::account::AccountId;
use crate::domain::shared::domain_error::DomainError;

pub struct Params {
    pub id: AccountId,
    pub new_password: String,
}

pub fn handler(context: &mut UseCaseContext, params: Params) -> Result<(), DomainError> {
    context
        .repositories
        .account_repository
        .find_by_id(params.id)
        .and_then(|account|
            context
                .services
                .domain
                .password_hashing_service
                .hash_password(params.new_password.as_str())
                .and_then(|hashed_password| account.update_password(hashed_password))
        )
        .and_then(|account| context.repositories.account_repository.save(account))
}
