pub mod account;
pub mod account_choice;
pub mod admin;
pub mod choice;
pub mod comment;
pub mod contest;
pub mod poll;
#[allow(unused_imports)]
pub(crate) mod schema;
pub mod types;

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool as PgPool, PooledConnection},
};

pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Pool {
    pool: PgPool<ConnectionManager<PgConnection>>,
}

impl Pool {
    pub fn new(url: impl Into<String>) -> Pool {
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = PgPool::new(manager).expect("Failed to create pg connection pool");
        Pool { pool }
    }

    pub async fn with_conn<T, F>(&self, func: F) -> anyhow::Result<T>
    where
        F: FnOnce(Connection) -> T + Send + 'static,
        T: Send + 'static,
    {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || Ok(func(pool.get()?))).await?
    }
}
