pub mod market;
pub mod organizer;
#[allow(unused_imports)]
mod schema;
pub mod types;
pub mod user;

use super::InfraFactory;
use diesel::pg::PgConnection;

pub trait PostgresInfra:
    market::PostgresMarketInfra + user::PostgresUserInfra + organizer::PostgresOrganizerInfra
{
    fn transaction<'a>(&'a self) -> Result<Transaction<'a>, failure::Error> {
        Transaction { postgres: self }
    }

    fn begin_transaction(&self) -> Result<(), failure::Error>;

    fn commit(&self) -> Result<(), failure::Error>;

    fn rollback(&self) -> Result<(), failure::Error>;
}

#[must_use]
pub struct Transaction<'a> {
    postgres: &'a dyn PostgresInfra,
    is_commit: bool,
}

impl<'a> Transaction<'a> {
    pub fn commit(&mut self) -> Result<(), failure::Error> {
        self.postgres.commit()?;
        self.is_commit = true;
    }
}

impl<'a> Drop for Transaction<'a> {
    fn drop(&mut self) {
        if !self.is_commit {
            self.postgres.rollback()
        }
    }
}

impl<'a> std::ops::Deref for Transaction<'a> {
    type Target = &'a dyn PostgresInfra;

    fn deref(&self) -> &Self::Target {
        &self.postgres
    }
}

/// 生成には `PostgresFactory` を使用する
///
/// ## Impls
///
/// - `PostgresMarketsInfra`
/// - `PostgresUsersInfra`
/// - `PostgresOrganizerInfra`
pub struct Postgres {
    conn: PgConnection,
}

impl PostgresInfra for Postgres {
    fn begin_transaction(&self) -> Result<(), failure::Error> {
        self.conn
            .transaction_manager()
            .begin_transaction(&self.conn)?;
        Ok(())
    }

    fn commit(&self) -> Result<(), failure::Error> {
        self.conn
            .transaction_manager()
            .commit_transaction(&self.conn)?;
        Ok(())
    }

    fn rollback(&self) -> Result<(), failure::Error> {
        self.conn
            .transaction_manager()
            .rollback_transaction(&self.conn)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PostgresFactory {
    url: String,
}

impl PostgresFactory {
    pub fn new(url: String) -> PostgresFactory {
        PostgresFactory { url: url }
    }
}

impl InfraFactory<Postgres> for PostgresFactory {
    fn create(&self) -> Result<Postgres, failure::Error> {
        Ok(Postgres {
            conn: PgConnection::establish(self.url.as_str())?,
        })
    }
}
