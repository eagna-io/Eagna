use libeagna::{app::ApiServer, infra::store::DbStoreFactory};
use log::info;

fn main() {
    env_logger::init();

    let pg_url = get_env_var_or_panic("PG_URL");
    let redis_url = get_env_var_or_panic("REDIS_URL");
    let firebase_api_key = get_env_var_or_panic("FIREBASE_API_KEY");
    let bind = get_env_var_or_panic("BIND");
    let access_allow_hosts = get_env_var_or_panic("ACCESS_ALLOW_HOSTS");

    let store_factory = DbStoreFactory::new(pg_url, redis_url, firebase_api_key);

    info!("Server is starting on {}", bind.as_str());

    ApiServer::new(store_factory, access_allow_hosts).run(bind.as_str());
}

fn get_env_var_or_panic(key: &'static str) -> String {
    std::env::var(key).expect(format!("{} is not specified", key).as_str())
}
