use std::sync::Arc;
use crate::app::_services::domain::date_manipulation::{ChronoDateManipulationService, DateManipulationAssembler};
use crate::app::_services::domain::id_generation::UuidIdGenerationService;
use crate::app::_services::domain::password_hashing::{BcryptPasswordHashingService, PasswordHashingAssembler};
use crate::app::_shared::application::use_case_context::{Assemblers, DomainServices, Factories, Repositories, Services, UseCaseContext};
use crate::app::account::application::AccountAssembler;
use crate::app::account::domain::AccountFactory;
use crate::app::account::infrastructure::PostgresqlAccountRepository;

fn init_client() -> postgres::Client {
    let postgres_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let postgres_password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let postgres_db = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let database_url = format!("postgres://{}:{}@localhost:5432/{}", postgres_user, postgres_password, postgres_db);

    postgres::Client::connect(&database_url, postgres::NoTls).expect("Error connecting to database")
}

pub fn new() -> UseCaseContext {
    let client = init_client();

    //services
    let password_hashing_service = Arc::new(BcryptPasswordHashingService {});
    let id_generation_service = Arc::new(UuidIdGenerationService {});
    let date_manipulation_service = Arc::new(ChronoDateManipulationService {});

    //assemblers
    let date_manipulation_assembler = Arc::new(DateManipulationAssembler {});
    let password_hashing_assembler = Arc::new(PasswordHashingAssembler {});

    UseCaseContext {
        services: Services {
            domain: DomainServices {
                password_hashing_service: password_hashing_service.clone(),
            }
        },
        repositories: Repositories {
            account_repository: Box::new(PostgresqlAccountRepository::new(client)),
        },
        factories: Factories {
            account_factory: AccountFactory::new(
                password_hashing_service,
                id_generation_service,
                date_manipulation_service.clone()
            ),
        },
        assemblers: Assemblers {
            account_assembler: AccountAssembler::new(
                password_hashing_assembler,
                date_manipulation_assembler,
                date_manipulation_service
            ),
        }
    }
}
