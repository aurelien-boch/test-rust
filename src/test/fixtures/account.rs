use std::time::Instant;
use crate::domain::account::{Account, AccountId};
use crate::domain::services::id_generation::Id;
use crate::domain::services::password_hashing::{PasswordHashingService};
use crate::test::mock::services::domain::MockPasswordHashingService;

pub fn get_account() -> Account {
    let hashing_service = MockPasswordHashingService::new();

    Account {
        id: AccountId::from(Id { value: "1".to_string() }),
        name: "Test Account".to_string(),
        email: "test-email@gmail.com".to_string(),
        password: hashing_service.hash_password("test-password").unwrap(),
        created_at: Instant::now(),
        updated_at: Instant::now(),
    }
}
