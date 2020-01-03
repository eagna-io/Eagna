pub mod market;
pub mod organizer;
#[allow(unused_imports)]
mod schema;
pub mod types;
pub mod user;

use super::InfraFactory;
use diesel::{
    connection::{Connection, TransactionManager},
    pg::PgConnection,
};

pub trait PostgresInfra:
    market::PostgresMarketInfra
    + user::PostgresUserInfra
    + organizer::PostgresOrganizerInfra
    + Send
    + 'static
{
    fn begin_transaction(&self) -> Result<(), failure::Error>;

    fn commit(&self) -> Result<(), failure::Error>;

    fn rollback(&self) -> Result<(), failure::Error>;
}

pub fn transaction<F, T, E>(pg: &dyn PostgresInfra, f: F) -> Result<T, E>
where
    F: FnOnce() -> Result<T, E>,
    E: From<failure::Error>,
{
    pg.begin_transaction()?;
    match f() {
        Ok(t) => {
            pg.commit()?;
            Ok(t)
        }
        Err(e) => {
            pg.rollback()?;
            Err(e)
        }
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
