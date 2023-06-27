use crate::app::_services::domain::id_generation::{Id, IdGenerationService};

pub struct UuidIdGenerationService {}

impl IdGenerationService for UuidIdGenerationService {
    fn generate(&self) -> Id {
        Id::new(uuid::Uuid::new_v4().to_string())
    }
}
