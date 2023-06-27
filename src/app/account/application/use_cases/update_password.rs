use crate::app::_shared::application::use_case_context::UseCaseContext;
use crate::app::_shared::domain::domain_error::DomainError;
use crate::app::account::domain::AccountId;

pub struct Params {
    pub id: AccountId,
    pub new_password: String,
}

pub fn handler(context: &mut UseCaseContext, params: Params) -> Result<(), DomainError> {
    context
        .repositories
        .account_repository
        .find_by_id(
            context
                .assemblers
                .account_assembler
                .dehydrate_account_id(&params.id)
        )
        .and_then(|result| result.ok_or(DomainError::not_found("Account not found".to_string())))
        .and_then(|account| context.assemblers.account_assembler.hydrate(&account))
        .and_then(|account|
            context
                .services
                .domain
                .password_hashing_service
                .hash_password(params.new_password.as_str())
                .and_then(|hashed_password| account.update_password(hashed_password))
        )
        .map(|account| context.assemblers.account_assembler.dehydrate(&account))
        .and_then(|account| context.repositories.account_repository.save(account))
}
