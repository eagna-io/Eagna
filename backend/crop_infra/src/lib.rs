#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

pub mod jwt;
pub mod mailgun;
pub mod postgres;

pub use self::postgres::{Postgres, PostgresFactory, PostgresInfra};

pub trait InfraFactory<Infra: Send + 'static>: Send + Sync + 'static {
    fn create(&self) -> anyhow::Result<Infra>;
}