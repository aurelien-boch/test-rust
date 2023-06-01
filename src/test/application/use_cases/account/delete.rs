use crate::domain::account::{AccountId};
use crate::test::fixtures::account::get_account;
use crate::test::mock::create_mock_context;

#[test]
fn delete_should_delete_the_account_with_the_given_id()
{
    let mut context = create_mock_context();
    let account = get_account();
    let params = crate::application::use_cases::account::delete::Params {
        id: account.id.clone()
    };

    context.repositories.account_repository.save(account.clone()).unwrap();
    crate::application::use_cases::account::delete::handler(&mut context, params).unwrap();
    assert!(context.repositories.account_repository.find_by_id(account.id).is_err());
}

#[test]
fn delete_should_fail_if_the_given_id_does_not_belong_to_any_account()
{
    let mut context = create_mock_context();
    let params = crate::application::use_cases::account::delete::Params { id: AccountId { value: "id".to_string() } };
    let result = crate::application::use_cases::account::delete::handler(&mut context, params);

    match result {
        Err(err) => {
            assert_eq!(err.error_type, crate::domain::shared::domain_error::DomainErrorCode::NotFound);
        },
        _ => { panic!("Expected error to be NotFound"); }
    }
}
