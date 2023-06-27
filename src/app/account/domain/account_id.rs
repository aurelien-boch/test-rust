use std::fmt::{Display, Formatter};
use crate::app::_services::domain::id_generation::Id;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountId {
    value: String
}

impl AccountId {
    pub fn from(id: Id) -> AccountId {
        AccountId {
            value: format!("account_{}", id.value())
        }
    }

    pub fn new(value: String) -> AccountId {
        AccountId {
            value
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Display for AccountId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccountId")
            .field("value", &self.value)
            .finish()
    }
}
