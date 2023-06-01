use crate::domain::services::password_hashing::PasswordHashingService;
use crate::infrastructure::services::domain::BcryptPasswordHashingService;

#[test]
fn bcrypt_password_hashing_should_hash_password_using_bcrypt_and_return_the_result() {
    let password = "password";
    let hashing_service = BcryptPasswordHashingService::new();
    let result = hashing_service.hash_password(password);

    assert!(bcrypt::verify("password", result.unwrap().value.as_str()).unwrap());
}

#[test]
fn bcrypt_password_hashing_should_compare_two_passwords_using_bcrypt_and_return_the_result() {
    let password = "password";
    let hashing_service = BcryptPasswordHashingService::new();
    let hashed_password = hashing_service.hash_password(password).unwrap();

    assert_eq!(
        hashing_service.verify_password(password, &hashed_password),
        bcrypt::verify(password, &hashed_password.value).unwrap()
    );
}
