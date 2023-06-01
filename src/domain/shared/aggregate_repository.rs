use crate::domain::shared::domain_error::DomainError;

pub trait AggregateRepository<T> {
    fn save(&mut self, aggregate: T) -> Result<(), DomainError>;
    fn delete(&mut self, aggregate: T) -> Result<(), DomainError>;
}
