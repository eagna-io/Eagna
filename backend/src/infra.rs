pub mod mailgun;
pub mod postgres;
pub mod redis;

pub use self::postgres::{Postgres, PostgresFactory, PostgresInfra};
pub use self::redis::{Redis, RedisFactory, RedisInfra};

pub trait InfraFactory<Infra: Send + 'static>: Send + Sync + 'static {
    fn create(&self) -> Result<Infra, failure::Error>;
}
