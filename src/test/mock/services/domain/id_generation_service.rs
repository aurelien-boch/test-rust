use crate::domain::services::id_generation::{Id, IdGenerationService};

pub struct MockIdGenerationService {}

impl IdGenerationService for MockIdGenerationService {
    fn generate(&self) -> Id {
        Id {
            value: "mock_id".to_string()
        }
    }
}

impl MockIdGenerationService {
    pub fn new() -> MockIdGenerationService {
        MockIdGenerationService {}
    }
}
