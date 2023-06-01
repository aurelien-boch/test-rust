use crate::domain::services::password_hashing::PasswordHashingService;
use crate::test::fixtures::account::get_account;
use crate::test::mock::create_mock_context;
use crate::test::mock::services::domain::MockPasswordHashingService;

#[test]
fn update_password_should_update_password() {
    let mut context = create_mock_context();
    let account = get_account();
    let new_password = "new-password".to_string();
    let password_service = MockPasswordHashingService::new();

    context.repositories.account_repository.save(account.clone()).unwrap();
    crate::application::use_cases::account::update_password::handler(
        &mut context,
        crate::application::use_cases::account::update_password::Params {
            id: account.id.clone(),
            new_password: new_password.clone(),
        },
    ).unwrap();

    let fetched_account = context.repositories.account_repository.find_by_id(account.id).unwrap();
    assert!(fetched_account.compare_password(&new_password, &password_service));
}

#[test]
fn update_password_should_fail_if_the_given_id_does_not_belong_to_any_account() {
    let mut context = create_mock_context();
    let result = crate::application::use_cases::account::update_password::handler(
        &mut context,
        crate::application::use_cases::account::update_password::Params {
            id: crate::domain::account::AccountId::from(crate::domain::services::id_generation::Id { value: "id".to_string() }),
            new_password: "new_password".to_string(),
        },
    );

    match result {
        Err(err) => {
            assert_eq!(err.error_type, crate::domain::shared::domain_error::DomainErrorCode::NotFound);
        }
        _ => { panic!("Expected error to be NotFound"); }
    }
}
