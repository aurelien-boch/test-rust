use crate::domain::account::{AccountId};
use crate::domain::services::id_generation::Id;
use crate::test::fixtures::account::get_account;

#[test]
fn update_name_should_update_name() {
    let mut context = crate::test::mock::create_mock_context();
    let account = get_account();
    let new_name = "new_name".to_string();

    context.repositories.account_repository.save(account.clone()).unwrap();
    crate::application::use_cases::account::update_name::handler(
        &mut context,
        crate::application::use_cases::account::update_name::Params {
            id: account.id.clone(),
            new_name: new_name.clone(),
        }).unwrap();

    let fetched_account = context.repositories.account_repository.find_by_id(account.id).unwrap();
    assert_eq!(fetched_account.name, new_name);
}

#[test]
fn update_name_should_fail_if_the_given_id_does_not_belong_to_any_account() {
    let mut context = crate::test::mock::create_mock_context();
    let result = crate::application::use_cases::account::update_name::handler(
        &mut context,
        crate::application::use_cases::account::update_name::Params {
            id: AccountId::from(Id { value: "id".to_string() }),
            new_name: "new_name".to_string(),
        });

    match result {
        Err(err) => {
            assert_eq!(err.error_type, crate::domain::shared::domain_error::DomainErrorCode::NotFound);
        },
        _ => { panic!("Expected error to be NotFound"); }
    }
}
