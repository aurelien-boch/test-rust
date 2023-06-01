use std::sync::Arc;

use crate::domain::account::{AccountFactory, AccountId};
use crate::domain::services::id_generation::IdGenerationService;
use crate::test::mock::services::domain::{MockIdGenerationService, MockPasswordHashingService};

#[test]
fn factory_should_create_account() {
    let password_hashing_service = Arc::new(MockPasswordHashingService::new());
    let mock_id_generation_service = Arc::new(MockIdGenerationService::new());

    let factory = AccountFactory::new(
        password_hashing_service.clone(),
            mock_id_generation_service.clone()
    );
    let account = factory.submit_account(
        "name".to_string(),
        "email".to_string(),
        "password".to_string()
    ).unwrap();

    assert_eq!(account.name, "name");
    assert_eq!(account.email, "email");
    assert_eq!(account.id, AccountId::from(mock_id_generation_service.generate()));
    assert!(account.password.compare("password", password_hashing_service.as_ref()));
}
