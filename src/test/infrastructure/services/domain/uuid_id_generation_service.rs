use crate::domain::services::id_generation::IdGenerationService;
use crate::infrastructure::services::domain::UuidIdGenerationService;

#[test]
fn uuid_id_generation_service_should_generate_a_new_uuid_id() {
    let id_generation_service = UuidIdGenerationService::new();
    let id = id_generation_service.generate();
    let id2 = id_generation_service.generate();

    assert_ne!(id.value, id2.value);
}
