use libeagna::app::{ApiServer, InfraManagerFactory};
use libeagna::infra::PostgresFactory;
use log::info;

fn main() {
    env_logger::init();

    // Postgresのセットアップ
    let pg_url = get_env_var_or_panic("PG_URL");
    let postgres_factory = PostgresFactory::new(pg_url);

    let infra_manager_factory = InfraManagerFactory::new(postgres_factory);

    let port = get_env_var_u16_or_panic("PORT");
    let access_allow_hosts = get_env_var_or_panic("ACCESS_ALLOW_HOSTS");

    info!("Server is starting on port {}", port);

    ApiServer::new(infra_manager_factory, access_allow_hosts).run(("0.0.0.0", port));
}

fn get_env_var_or_panic(key: &'static str) -> String {
    std::env::var(key).expect(format!("{} is not specified", key).as_str())
}

fn get_env_var_u16_or_panic(key: &'static str) -> u16 {
    let s = get_env_var_or_panic(key);
    u16::from_str_radix(s.as_str(), 10).unwrap()
}
