use std::cell::Cell;
use crate::app::_shared::domain::aggregate_repository::AggregateRepository;
use crate::app::_shared::domain::domain_error::DomainError;
use crate::app::account::domain::{AccountRepository};
use postgres::error::SqlState;
use crate::app::_services::domain::date_manipulation::UtcInstantDto;
use crate::app::_services::domain::password_hashing::HashedPasswordDto;
use crate::app::account::application::dto::{AccountDto, AccountIdDto};

pub struct PostgresqlAccountRepository {
    client: Cell<postgres::Client>,
}

impl AggregateRepository<AccountDto> for PostgresqlAccountRepository {
    fn save(&mut self, aggregate: AccountDto) -> Result<(), DomainError> {
        self
            .client
            .get_mut()
            .execute(
                "
                INSERT INTO account (id, name, email, password, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5,$6)
                ON CONFLICT (id) DO UPDATE
                    SET id = excluded.id,
                        name = excluded.name,
                        email = excluded.email,
                        password = excluded.password,
                        created_at = excluded.created_at,
                        updated_at = excluded.updated_at;
                ",
                &[
                    &aggregate.id.value,
                    &aggregate.name,
                    &aggregate.email,
                    &aggregate.password.value,
                    &aggregate.created_at.date,
                    &aggregate.updated_at.date
                ],
            )
            .map_err(|err| Self::parse_create_error(err.code().unwrap()))
            .and_then(|rows| {
                if rows == 0 {
                    Err(DomainError::unknown("Error while saving user".to_string()))
                } else {
                    Ok(())
                }
            })
    }
}

impl AccountRepository for PostgresqlAccountRepository {
    fn find_by_id(&mut self, id: AccountIdDto) -> Result<Option<AccountDto>, DomainError> {
        self
            .client
            .get_mut()
            .query(
                "SELECT * FROM account WHERE id = $1",
                &[&id.value],
            )
            .map_err(|err| DomainError::service_unavailable(err.to_string()))
            .map(|rows| {
                rows
                    .iter()
                    .map(|row| {
                        AccountDto {
                            id: AccountIdDto {
                                value: row.get(0),
                            },
                            name: row.get(1),
                            email: row.get(2),
                            password: HashedPasswordDto { value: row.get(3) },
                            created_at: UtcInstantDto { date: row.get(4) },
                            updated_at: UtcInstantDto { date: row.get(5) },
                        }
                    })
                    .next()
            })
    }

    fn is_email_already_used(&mut self, email: &str) -> Result<bool, DomainError> {
        self
            .client
            .get_mut()
            .query(
                "SELECT count(*) FROM account WHERE email = $1",
                &[&email],
            )
            .map_err(|err| DomainError::service_unavailable(err.to_string()))
            .and_then(
                |rows| {
                    rows
                        .iter()
                        .map(|row| row.get(0))
                        .next()
                        .ok_or_else(|| DomainError::unknown("Error while checking if email is already used".to_string()))
                },
            )
    }
}

impl PostgresqlAccountRepository {
    fn parse_create_error(err: &SqlState) -> DomainError {
        match err {
            &SqlState::UNIQUE_VIOLATION => DomainError::already_exists("Email already used".to_string()),
            code => DomainError::unknown(
                format!(
                    "Error while saving user. Postgresql error code: {}",
                    code.code()
                )
            ),
        }
    }

    pub fn new(
        client: postgres::Client,
    ) -> Self {
        Self {
            client: Cell::new(client)
        }
    }
}
