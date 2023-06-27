use crate::app::_services::domain::date_manipulation::{UtcInstantDto};
use crate::app::_services::domain::password_hashing::HashedPasswordDto;
use crate::app::account::application::dto::AccountIdDto;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountDto {
    pub id: AccountIdDto,
    pub name: String,
    pub email: String,
    pub password: HashedPasswordDto,
    pub created_at: UtcInstantDto,
    pub updated_at: UtcInstantDto,
}
