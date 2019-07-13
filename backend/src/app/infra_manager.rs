use crate::infra::{FirebaseInfra, InfraFactory, PostgresInfra, RedisInfra};
use std::sync::Arc;

pub struct InfraManager {
    firebase: LazyCache<Box<dyn FirebaseInfra>>,
    redis: LazyCache<Box<dyn RedisInfra>>,
    postgres: LazyCache<Box<dyn PostgresInfra>>,
}

impl InfraManager {
    pub fn get_firebase(&self) -> Result<&dyn FirebaseInfra, failure::Error> {
        self.firebase.get().map(|i| i.as_ref())
    }

    pub fn get_redis(&self) -> Result<&dyn RedisInfra, failure::Error> {
        self.redis.get().map(|i| i.as_ref())
    }

    pub fn get_postgres(&self) -> Result<&dyn PostgresInfra, failure::Error> {
        self.postgres.get().map(|i| i.as_ref())
    }
}

pub struct InfraManagerFactory {
    firebase_factory: Arc<dyn InfraFactory<Box<dyn FirebaseInfra>>>,
    redis_factory: Arc<dyn InfraFactory<Box<dyn RedisInfra>>>,
    postgres_factory: Arc<dyn InfraFactory<Box<dyn PostgresInfra>>>,
}

impl InfraManagerFactory {
    pub fn new<FBF, FB, RDF, RD, PGF, PG>(
        firebase_factory: FBF,
        redis_factory: RDF,
        postgres_factory: PGF,
    ) -> InfraManagerFactory
    where
        FBF: InfraFactory<FB>,
        FB: FirebaseInfra,
        RDF: InfraFactory<RD>,
        RD: RedisInfra,
        PGF: InfraFactory<PG>,
        PG: PostgresInfra,
    {
        InfraManagerFactory {
            firebase_factory: Arc::new(BoxingInfraFactory::new(firebase_factory)),
            redis_factory: Arc::new(BoxingInfraFactory::new(redis_factory)),
            postgres_factory: Arc::new(BoxingInfraFactory::new(postgres_factory)),
        }
    }

    pub fn create(&self) -> InfraManager {
        InfraManager {
            firebase: LazyCache::new(self.firebase_factory.clone()),
            redis: LazyCache::new(self.redis_factory.clone()),
            postgres: LazyCache::new(self.postgres_factory.clone()),
        }
    }
}

pub struct LazyCache<I> {
    factory: Arc<dyn InfraFactory<I>>,
    infra: Option<I>,
}

impl<I> LazyCache<I> {
    pub fn new(factory: Arc<dyn InfraFactory<I>>) -> LazyCache<I> {
        LazyCache {
            factory,
            infra: None,
        }
    }

    pub fn get(&self) -> Result<&I, failure::Error> {
        if let Some(ref infra) = self.infra {
            return Ok(infra);
        }

        self.infra = Some(self.factory.create()?);
        Ok(self.infra.as_ref().unwrap())
    }
}

struct BoxingInfraFactory<F: ?Sized> {
    factory: F,
}

impl<F: ?Sized> BoxingInfraFactory<F> {
    fn new(factory: F) -> BoxingInfraFactory<F> {
        BoxingInfraFactory { factory }
    }
}

impl<F, I> InfraFactory<Box<I>> for BoxingInfraFactory<F>
where
    F: InfraFactory<I>,
{
    fn create(&self) -> Result<Box<I>, failure::Error> {
        Ok(Box::new(self.factory.create()?))
    }
}
