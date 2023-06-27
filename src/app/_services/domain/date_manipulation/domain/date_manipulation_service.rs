use super::UtcInstant;

pub trait DateManipulationService {
    fn now(&self) -> UtcInstant;
    fn is_after(&self, first: &UtcInstant, second: &UtcInstant) -> bool;
}
