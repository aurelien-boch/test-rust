use crate::app::_shared::domain::domain_error::DomainError;

pub trait AggregateRepository<AggregateDto> {
    fn save(&mut self, aggregate: AggregateDto) -> Result<(), DomainError>;
}
