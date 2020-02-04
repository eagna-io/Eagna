use crate::infra::{InfraFactory, PostgresInfra};
use lazycell::LazyCell;
use std::sync::Arc;

pub struct InfraManager {
    postgres: LazyInfra<Box<dyn PostgresInfra>>,
}

impl InfraManager {
    pub fn get_postgres(&self) -> anyhow::Result<&dyn PostgresInfra> {
        self.postgres.get().map(|i| i.as_ref())
    }
}

pub struct InfraManagerFactory {
    postgres_factory: Arc<dyn InfraFactory<Box<dyn PostgresInfra>>>,
}

impl InfraManagerFactory {
    pub fn new<PGF, PG>(postgres_factory: PGF) -> InfraManagerFactory
    where
        PGF: InfraFactory<PG>,
        PG: PostgresInfra,
    {
        InfraManagerFactory {
            postgres_factory: Arc::new(BoxingInfraFactory::new(postgres_factory)),
        }
    }

    pub fn create(&self) -> InfraManager {
        InfraManager {
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

    pub fn get(&self) -> anyhow::Result<&I> {
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
    fn create(&self) -> anyhow::Result<Box<dyn PostgresInfra>> {
        Ok(Box::new(self.factory.create()?))
    }
}
