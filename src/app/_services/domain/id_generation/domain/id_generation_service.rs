use crate::app::_services::domain::id_generation::Id;

pub trait IdGenerationService {
    fn generate(&self) -> Id;
}
