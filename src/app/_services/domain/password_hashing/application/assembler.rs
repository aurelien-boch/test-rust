use crate::app::_services::domain::password_hashing::{HashedPassword, HashedPasswordDto};

pub struct PasswordHashingAssembler {}

impl PasswordHashingAssembler {
    pub fn hydrate(&self, hashed_password: &HashedPasswordDto) -> HashedPassword {
        HashedPassword {
            value: hashed_password.value.to_string(),
        }
    }

    pub fn dehydrate(&self, hashed_password: &HashedPassword) -> HashedPasswordDto {
        HashedPasswordDto {
            value: hashed_password.value.to_string(),
        }
    }
}
