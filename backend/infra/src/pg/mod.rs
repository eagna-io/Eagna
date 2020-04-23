use async_trait::async_trait;
use domain::account::{Account, AccountRepository};
use domain::contest::{Contest, ContestId, ContestRepository};

mod account;
mod contest;
#[allow(unused_imports)]
mod schema;
pub mod types;

use account::save_account;
use contest::{find_contest_by_id, save_contest};

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool as PgPool, PooledConnection},
};

type Connection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct Postgres {
    pool: PgPool<ConnectionManager<PgConnection>>,
}

impl Postgres {
    pub fn new(url: impl Into<String>) -> Postgres {
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = PgPool::new(manager).unwrap();
        Postgres { pool }
    }

    async fn with_conn<T, F>(&self, func: F) -> anyhow::Result<T>
    where
        F: FnOnce(Connection) -> T + Send + 'static,
        T: Send + 'static,
    {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || Ok(func(pool.get()?))).await?
    }

    async fn try_with_conn<T, F>(&self, func: F) -> anyhow::Result<T>
    where
        F: FnOnce(Connection) -> anyhow::Result<T> + Send + 'static,
        T: Send + 'static,
    {
        self.with_conn(func).await?
    }
}

#[async_trait]
impl AccountRepository for Postgres {
    async fn save(&mut self, account: Account) -> anyhow::Result<()> {
        self.try_with_conn(move |conn| save_account(&conn, &account))
            .await
    }
}

#[async_trait]
impl ContestRepository for Postgres {
    async fn find_by_id(&mut self, id: ContestId) -> anyhow::Result<Option<Contest>> {
        self.try_with_conn(move |conn| find_contest_by_id(&conn, id))
            .await
    }

    async fn save(&mut self, contest: Contest) -> anyhow::Result<()> {
        self.try_with_conn(move |conn| save_contest(&conn, &contest))
            .await
    }
}
