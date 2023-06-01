use std::fmt::{Display, Formatter};
use crate::domain::services::id_generation::Id;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountId {
    pub value: String
}

impl AccountId {
    pub fn from(id: Id) -> AccountId {
        AccountId {
            value: format!("account_{}", id.value)
        }
    }
}

impl Display for AccountId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccountId")
            .field("value", &self.value)
            .finish()
    }
}
