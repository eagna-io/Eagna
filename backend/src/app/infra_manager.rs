use crate::infra::{InfraFactory, PostgresInfra, RedisInfra};
use lazycell::LazyCell;
use std::sync::Arc;

pub struct InfraManager {
    redis: LazyInfra<Box<dyn RedisInfra>>,
    postgres: LazyInfra<Box<dyn PostgresInfra>>,
}

impl InfraManager {
    pub fn get_redis(&self) -> Result<&dyn RedisInfra, failure::Error> {
        self.redis.get().map(|i| i.as_ref())
    }

    pub fn get_postgres(&self) -> Result<&dyn PostgresInfra, failure::Error> {
        self.postgres.get().map(|i| i.as_ref())
    }
}

pub struct InfraManagerFactory {
    redis_factory: Arc<dyn InfraFactory<Box<dyn RedisInfra>>>,
    postgres_factory: Arc<dyn InfraFactory<Box<dyn PostgresInfra>>>,
}

impl InfraManagerFactory {
    pub fn new<RDF, RD, PGF, PG>(redis_factory: RDF, postgres_factory: PGF) -> InfraManagerFactory
    where
        RDF: InfraFactory<RD>,
        RD: RedisInfra,
        PGF: InfraFactory<PG>,
        PG: PostgresInfra,
    {
        InfraManagerFactory {
            redis_factory: Arc::new(BoxingInfraFactory::new(redis_factory)),
            postgres_factory: Arc::new(BoxingInfraFactory::new(postgres_factory)),
        }
    }

    pub fn create(&self) -> InfraManager {
        InfraManager {
            redis: LazyInfra::new(self.redis_factory.clone()),
            postgres: LazyInfra::new(self.postgres_factory.clone()),
        }
    }
}

pub struct LazyInfra<I> {
    factory: Arc<dyn InfraFactory<I>>,
    infra: LazyCell<I>,
}

impl<I: Send + 'static> LazyInfra<I> {
    pub fn new(factory: Arc<dyn InfraFactory<I>>) -> LazyInfra<I> {
        LazyInfra {
            factory,
            infra: LazyCell::new(),
        }
    }

    pub fn get(&self) -> Result<&I, failure::Error> {
        if !self.infra.filled() {
            let _never_err = self.infra.fill(self.factory.create()?);
        }
        Ok(self.infra.borrow().unwrap())
    }
}

struct BoxingInfraFactory<F, I> {
    factory: F,
    _phantom: std::marker::PhantomData<I>,
}

unsafe impl<F, I> Sync for BoxingInfraFactory<F, I> {}

impl<F, I> BoxingInfraFactory<F, I>
where
    F: InfraFactory<I>,
    I: Send + 'static,
{
    fn new(factory: F) -> BoxingInfraFactory<F, I> {
        BoxingInfraFactory {
            factory,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<F, I> InfraFactory<Box<dyn PostgresInfra>> for BoxingInfraFactory<F, I>
where
    F: InfraFactory<I>,
    I: PostgresInfra,
{
    fn create(&self) -> Result<Box<dyn PostgresInfra>, failure::Error> {
        Ok(Box::new(self.factory.create()?))
    }
}

impl<F, I> InfraFactory<Box<dyn RedisInfra>> for BoxingInfraFactory<F, I>
where
    F: InfraFactory<I>,
    I: RedisInfra,
{
    fn create(&self) -> Result<Box<dyn RedisInfra>, failure::Error> {
        Ok(Box::new(self.factory.create()?))
    }
}
