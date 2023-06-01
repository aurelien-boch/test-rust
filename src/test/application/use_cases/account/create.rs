use crate::test::mock::create_mock_context;

#[test]
fn create_should_create_an_account_and_store_it_in_repository()
{
    let mut context = create_mock_context();
    let params = crate::application::use_cases::account::create::Params {
        email: "a@b.c".to_string(),
        password: "hi".to_string(),
        name: "hi".to_string()
    };

    crate::application::use_cases::account::create::handler(&mut context, params).unwrap();
    //TODO: ensure that the account was created and stored in the repository
}

#[test]
fn create_should_fail_with_conflict_if_email_already_exists()
{
    let mut context = create_mock_context();
    let params = crate::application::use_cases::account::create::Params {
        email: "a@b.c".to_string(),
        password: "hi".to_string(),
        name: "hi".to_string()
    };
    let params_copy = crate::application::use_cases::account::create::Params {
        email: "a@b.c".to_string(),
        password: "hi".to_string(),
        name: "hi".to_string()
    };

    crate::application::use_cases::account::create::handler(&mut context, params).unwrap();
    let result = crate::application::use_cases::account::create::handler(&mut context, params_copy);

    match result {
        Err(error) => {
            assert_eq!(error.error_type, crate::domain::shared::domain_error::DomainErrorCode::Conflict);
        },
        _ => {
            panic!("Expected error to be Conflict");
        }
    }
}
