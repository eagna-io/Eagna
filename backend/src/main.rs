use libeagna::app::{ApiServer, InfraManagerFactory};
use libeagna::infra::{FirebaseFactory, PostgresFactory, RedisFactory};
use log::info;

fn main() {
    env_logger::init();

    let pg_url = get_env_var_or_panic("PG_URL");
    let redis_url = get_env_var_or_panic("REDIS_URL");
    let firebase_api_key = get_env_var_or_panic("FIREBASE_API_KEY");
    let bind = get_env_var_or_panic("BIND");
    let access_allow_hosts = get_env_var_or_panic("ACCESS_ALLOW_HOSTS");

    let firebase_factory = FirebaseFactory::new(firebase_api_key);
    let redis_factory = RedisFactory::new(redis_url);
    let postgres_factory = PostgresFactory::new(pg_url);

    let infra_manager_factory =
        InfraManagerFactory::new(firebase_factory, redis_factory, postgres_factory);

    info!("Server is starting on {}", bind.as_str());

    ApiServer::new(infra_manager_factory, access_allow_hosts).run(bind.as_str());
}

fn get_env_var_or_panic(key: &'static str) -> String {
    std::env::var(key).expect(format!("{} is not specified", key).as_str())
}
