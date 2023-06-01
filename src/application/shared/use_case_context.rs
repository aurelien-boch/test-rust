use std::sync::Arc;
use crate::domain::account::{AccountFactory, AccountRepository};
use crate::domain::services::password_hashing::PasswordHashingService;


pub struct Repositories {
    pub account_repository: Box<dyn AccountRepository>,
}

pub struct Factories {
    pub account_factory: AccountFactory,
}

pub struct DomainServices {
    pub password_hashing_service: Arc<dyn PasswordHashingService>,
}

pub struct Services {
    pub domain: DomainServices,
}

pub struct UseCaseContext {
    pub factories: Factories,
    pub services: Services,
    pub repositories: Repositories,
}
