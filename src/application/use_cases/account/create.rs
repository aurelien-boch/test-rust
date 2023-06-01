use crate::application::shared::UseCaseContext;
use crate::domain::shared::domain_error::{DomainError, DomainErrorCode};

pub struct Params {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub fn handler(context: &mut UseCaseContext, params: Params) -> Result<(), DomainError> {
    context.repositories.account_repository
        .is_email_already_used(&params.email)
        .and_then(|result|
            match result {
                true => Err(DomainError::new(
                    "Email already used".to_string(),
                    DomainErrorCode::Conflict,
                )),
                false => Ok(())
            }
        )
        .and_then(|_|
            context.factories.account_factory
                .submit_account(
                    params.name,
                    params.email,
                    params.password,
                )
                .and_then(
                    |account| context.repositories.account_repository.save(account)
                )
        )
}
