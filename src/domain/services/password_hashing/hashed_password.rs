use crate::domain::services::password_hashing::PasswordHashingService;

#[derive(Clone)]
pub struct HashedPassword {
    pub value: String
}

impl HashedPassword {
    pub fn new(value: String) -> HashedPassword {
        HashedPassword {
            value
        }
    }

    pub fn compare(&self, password: &str, hashing_service: &dyn PasswordHashingService) -> bool {
        hashing_service.verify_password(password, self)
    }
}
