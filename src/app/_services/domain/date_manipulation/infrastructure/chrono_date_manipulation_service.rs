use crate::app::_services::domain::date_manipulation::DateManipulationService;
use super::super::{UtcInstant};

pub struct ChronoDateManipulationService {}

impl ChronoDateManipulationService {
    pub fn new() -> Self {
        Self {}
    }
}

impl DateManipulationService for ChronoDateManipulationService {
    fn now(&self) -> UtcInstant {
        UtcInstant {
            date: chrono::Utc::now()
        }
    }

    fn is_after(&self, first: &UtcInstant, second: &UtcInstant) -> bool {
        first.date > second.date
    }
}
