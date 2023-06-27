use crate::app::_shared::application::use_case_context::UseCaseContext;
use crate::app::_shared::domain::domain_error::DomainError;

pub struct Params {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub fn handler(context: &mut UseCaseContext, params: Params) -> Result<(), DomainError> {
    context
        .repositories
        .account_repository
        .is_email_already_used(&params.email)
        .and_then(|result|
            match result {
                true => Err(DomainError::already_exists("Email already used".to_string())),
                false => Ok(())
            }
        )
        .and_then(|_|
            context
                .factories
                .account_factory
                .submit_account(
                    params.name,
                    params.email,
                    params.password,
                )
        )
        .map(|account| context.assemblers.account_assembler.dehydrate(&account))
        .and_then(
            |account| context
                .repositories
                .account_repository
                .save(account)
        )
}
