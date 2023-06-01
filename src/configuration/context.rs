use std::sync::Arc;
use crate::application::shared::{DomainServices, Factories, Repositories, Services, UseCaseContext};
use crate::domain::account::AccountFactory;
use crate::infrastructure::persistence::memory::InMemoryAccountRepository;
use crate::infrastructure::services::domain::{BcryptPasswordHashingService, UuidIdGenerationService};

pub fn init_context() -> UseCaseContext {
    let id_generation_service = Arc::new(UuidIdGenerationService::new());
    let password_hashing_service = Arc::new(BcryptPasswordHashingService::new());

    UseCaseContext {
        factories: Factories {
            account_factory: AccountFactory::new(
                password_hashing_service.clone(),
                id_generation_service,
            ),
        },
        services: Services {
            domain: DomainServices {
                password_hashing_service,
            },
        },
        repositories: Repositories {
            account_repository: Box::new(InMemoryAccountRepository::new()),
        },
    }
}
