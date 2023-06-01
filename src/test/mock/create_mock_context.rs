use std::sync::Arc;
use crate::application::shared::{DomainServices, Factories, Repositories, Services, UseCaseContext};
use crate::domain::account::AccountFactory;
use crate::test::mock::repositories::MockAccountRepository;
use crate::test::mock::services::domain::{MockIdGenerationService, MockPasswordHashingService};

pub fn create_mock_context() -> UseCaseContext {
    let password_hashing_service = Arc::new(MockPasswordHashingService::new());
    let id_generation_service = Arc::new(MockIdGenerationService::new());

    UseCaseContext {
        services: Services {
            domain: DomainServices {
                password_hashing_service: password_hashing_service.clone()
            }
        },
        factories: Factories {
            account_factory: AccountFactory::new(
                password_hashing_service,
                id_generation_service
            )
        },
        repositories: Repositories {
            account_repository: Box::new(MockAccountRepository::new())
        }
    }
}
