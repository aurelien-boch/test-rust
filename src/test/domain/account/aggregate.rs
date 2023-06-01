use std::thread::sleep;
use std::time::Duration;

use crate::domain::services::password_hashing::PasswordHashingService;
use crate::test::fixtures::account::get_account;
use crate::test::mock::services::domain::MockPasswordHashingService;

#[test]
fn update_name_should_update_name() {
    let account = get_account();
    let new_name = "new-name".to_string();
    let updated_account = account.update_name(new_name.clone());

    assert_eq!(updated_account.unwrap().name, new_name);
}

#[test]
fn update_email_should_update_email() {
    let account = get_account();
    let new_email = "new-email@gmail.com".to_string();
    let updated_account = account.update_email(new_email.clone());

    assert_eq!(updated_account.unwrap().email, new_email);
}

#[test]
fn update_password_should_update_password() {
    let account = get_account();
    let new_password = "new-password".to_string();
    let password_service = MockPasswordHashingService::new();
    let hashed = password_service.hash_password(new_password.as_str());
    let updated_account = account.update_password(hashed.unwrap());

    assert!(updated_account.unwrap().compare_password(&new_password, &password_service));
}

#[test]
fn update_methods_should_update_updated_at_time() {
    let account = get_account();
    let new_name = "new-name".to_string();
    sleep(Duration::new(0, 1));
    let updated_account = account.update_name(new_name);

    assert!(updated_account.unwrap().updated_at > account.updated_at);
}

#[test]
fn compare_password_should_return_true_when_password_is_correct() {
    let account = get_account();
    let password_service = MockPasswordHashingService::new();

    assert!(account.compare_password(&account.password.value, &password_service));
}

#[test]
fn compare_password_should_return_false_when_password_is_incorrect() {
    let account = get_account();
    let password = "incorrect-password".to_string();
    let password_service = MockPasswordHashingService::new();

    assert!(!account.compare_password(&password, &password_service));
}
