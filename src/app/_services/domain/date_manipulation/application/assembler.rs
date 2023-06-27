use crate::app::_services::domain::date_manipulation::{UtcInstant, UtcInstantDto};
use crate::app::_shared::domain::domain_error::DomainError;

pub struct DateManipulationAssembler {}

impl DateManipulationAssembler {
    pub fn hydrate(&self, utc_instant_dto: &UtcInstantDto) -> Result<UtcInstant, DomainError> {
        Ok(
            UtcInstant {
                date: utc_instant_dto.date.parse::<chrono::DateTime<chrono::Utc>>()
                    .map_err(|err| DomainError::unknown(format!("Error parsing date: {}", err)))?
            }
        )
    }

    pub fn dehydrate(&self, utc_instant: &UtcInstant) -> UtcInstantDto {
        UtcInstantDto {
            date: utc_instant.date.to_rfc3339()
        }
    }
}
