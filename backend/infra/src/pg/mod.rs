use lazycell::AtomicLazyCell;

mod account;
mod comment;
mod contest;
#[allow(unused_imports)]
mod schema;
pub mod types;

pub use account::AccountRepository;
pub use comment::CommentRepository;
pub use contest::ContestRepository;

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool as PgPool, PooledConnection},
};

pub fn initialize_global_pg(url: impl Into<String>) {
    let pg = Postgres::new(url);
    GLOBAL_PG
        .inner
        .fill(pg)
        .map_err(drop)
        .expect("Already initialized")
}

pub static GLOBAL_PG: GlobalPostgres = GlobalPostgres {
    inner: AtomicLazyCell::NONE,
};

pub struct GlobalPostgres {
    inner: AtomicLazyCell<Postgres>,
}

impl AsRef<Postgres> for GlobalPostgres {
    fn as_ref(&self) -> &Postgres {
        self.inner.borrow().unwrap()
    }
}

/*
 * =========
 * Internals
 * =========
 */
type Connection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
struct Postgres {
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
