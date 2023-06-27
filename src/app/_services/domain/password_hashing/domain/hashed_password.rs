use std::fmt::{Display, Formatter};
use super::PasswordHashingService;

#[derive(Clone)]
pub struct HashedPassword {
    pub(in super::super::super) value: String,
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

impl Display for HashedPassword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let stars = "**********";

        f.debug_struct("HashedPassword")
            .field("value", &stars)
            .finish()
    }
}
