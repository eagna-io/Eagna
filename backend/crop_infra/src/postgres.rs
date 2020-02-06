pub mod market;
pub mod order;
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
    market::PostgresMarketInfra + user::PostgresUserInfra + order::PostgresOrderInfra + Send + 'static
{
    fn begin_transaction(&self) -> anyhow::Result<()>;

    fn commit(&self) -> anyhow::Result<()>;

    fn rollback(&self) -> anyhow::Result<()>;
}

pub fn transaction<F, T, E>(pg: &dyn PostgresInfra, f: F) -> Result<T, E>
where
    F: FnOnce() -> anyhow::Result<T, E>,
    E: From<anyhow::Error>,
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
    fn begin_transaction(&self) -> anyhow::Result<()> {
        self.conn
            .transaction_manager()
            .begin_transaction(&self.conn)?;
        Ok(())
    }

    fn commit(&self) -> anyhow::Result<()> {
        self.conn
            .transaction_manager()
            .commit_transaction(&self.conn)?;
        Ok(())
    }

    fn rollback(&self) -> anyhow::Result<()> {
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
    fn create(&self) -> anyhow::Result<Postgres> {
        Ok(Postgres {
            conn: PgConnection::establish(self.url.as_str())?,
        })
    }
}
