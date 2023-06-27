use std::fmt::Display;
use chrono::{Utc};

#[derive(Copy, Clone, Debug)]
pub struct UtcInstant {
    pub (in super::super::super) date: chrono::DateTime<Utc>
}

impl Display for UtcInstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.date.to_rfc3339())
    }
}
