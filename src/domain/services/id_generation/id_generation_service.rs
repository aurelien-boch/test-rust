use crate::domain::services::id_generation::Id;

pub trait IdGenerationService {
    fn generate(&self) -> Id;
}
