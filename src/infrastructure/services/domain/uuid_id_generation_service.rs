use crate::domain::services::id_generation::{Id, IdGenerationService};

pub struct UuidIdGenerationService {}

impl UuidIdGenerationService {
    pub fn new() -> UuidIdGenerationService {
        UuidIdGenerationService {}
    }
}

impl IdGenerationService for UuidIdGenerationService {
    fn generate(&self) -> Id {
        Id {
            value: uuid::Uuid::new_v4().to_string()
        }
    }
}
