use std::sync::Arc;
use crate::app::account;
use crate::app::_services;

pub struct Factories {
    pub account_factory: account::domain::AccountFactory,
}

pub struct Repositories {
    pub account_repository: Box<dyn account::domain::AccountRepository>,
}

pub struct DomainServices {
    pub password_hashing_service: Arc<dyn _services::domain::password_hashing::PasswordHashingService>,
}

pub struct Services {
    pub domain: DomainServices,
}

pub struct Assemblers {
    pub account_assembler: account::application::AccountAssembler,
}

pub struct UseCaseContext {
    pub factories: Factories,
    pub repositories: Repositories,
    pub services: Services,
    pub assemblers: Assemblers,
}
