pub mod firebase;
pub mod postgres;
pub mod redis;

pub use self::firebase::{Firebase, FirebaseInfra};
pub use self::postgres::{Postgres, PostgresInfra};
pub use self::redis::{Redis, RedisInfra};

pub trait InfraFactory<Infra> {
    fn create(&self) -> Result<Infra, failure::Error>;
}
