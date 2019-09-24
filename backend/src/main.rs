use libeagna::app::{ApiServer, InfraManagerFactory};
use libeagna::infra::{FirebaseFactory, MockFirebaseFactory, PostgresFactory, RedisFactory};
use log::info;
use std::collections::HashMap;

fn main() {
    env_logger::init();

    match RunMode::from_str(get_env_var_or("RUN_MODE", "develop")) {
        RunMode::Production => run_in_production_mode(),
        RunMode::Develop => run_in_develop_mode(),
        RunMode::Test => run_in_test_mode(),
    }
}

fn run_in_production_mode() {
    info!("RUN in production mode");

    // Postgresのセットアップ
    let pg_url = get_env_var_or_panic("PG_URL");
    let postgres_factory = PostgresFactory::new(pg_url);

    // Redisのセットアップ
    let redis_url = get_env_var_or_panic("REDIS_URL");
    let redis_factory = RedisFactory::new(redis_url);

    // Firebaseのセットアップ
    let firebase_api_key = get_env_var_or_panic("FIREBASE_API_KEY");
    let firebase_factory = FirebaseFactory::new(firebase_api_key);

    let bind = get_env_var_or_panic("BIND");
    let access_allow_hosts = get_env_var_or_panic("ACCESS_ALLOW_HOSTS");

    let infra_manager_factory =
        InfraManagerFactory::new(firebase_factory, redis_factory, postgres_factory);

    info!("Server is starting on {}", bind.as_str());

    ApiServer::new(infra_manager_factory, access_allow_hosts).run(bind.as_str());
}

fn run_in_develop_mode() {
    info!("RUN in develop mode");

    // Postgresのセットアップ
    let pg_url = get_env_var_or("PG_URL", "postgres://postgres:postgres@localhost");
    let postgres_factory = PostgresFactory::new(pg_url);

    // Redisのセットアップ
    let redis_url = get_env_var_or("REDIS_URL", "redis://localhost");
    let redis_factory = RedisFactory::new(redis_url);

    // Firebaseのセットアップ
    let firebase_api_key = get_env_var_or_panic("FIREBASE_API_KEY");
    let firebase_factory = FirebaseFactory::new(firebase_api_key);

    let bind = get_env_var_or("BIND", "0.0.0.0:8080");
    let access_allow_hosts = get_env_var_or("ACCESS_ALLOW_HOSTS", "*");

    let infra_manager_factory =
        InfraManagerFactory::new(firebase_factory, redis_factory, postgres_factory);

    info!("Server is starting on {}", bind.as_str());

    ApiServer::new(infra_manager_factory, access_allow_hosts).run(bind.as_str());
}

fn run_in_test_mode() {
    info!("RUN in test mode");

    // Postgresのセットアップ
    let pg_url = get_env_var_or_panic("PG_URL");
    let postgres_factory = PostgresFactory::new(pg_url);

    // Redisのセットアップ
    let redis_url = get_env_var_or_panic("REDIS_URL");
    let redis_factory = RedisFactory::new(redis_url);

    // Firebaseのセットアップ
    let mut test_data = HashMap::new();
    test_data.insert("test_user_access_token".into(), "test_user".into());
    test_data.insert("test_admin_access_token".into(), "test_admin".into());
    let firebase_factory = MockFirebaseFactory::new(test_data);

    let bind = get_env_var_or_panic("BIND");
    let access_allow_hosts = "*".into();

    let infra_manager_factory =
        InfraManagerFactory::new(firebase_factory, redis_factory, postgres_factory);

    info!("Server is starting on {}", bind.as_str());

    ApiServer::new(infra_manager_factory, access_allow_hosts).run(bind.as_str());
}

#[derive(PartialEq, Eq)]
enum RunMode {
    Develop,
    Production,
    Test,
}

impl RunMode {
    fn from_str(s: String) -> Self {
        match s.to_uppercase().as_str() {
            "DEVELOP" => RunMode::Develop,
            "PRODUCTION" => RunMode::Production,
            "TEST" => RunMode::Test,
            _ => panic!(format!("{} is unsupported RUN_MODE", s.clone())),
        }
    }
}

fn get_env_var_or_panic(key: &'static str) -> String {
    std::env::var(key).expect(format!("{} is not specified", key).as_str())
}

fn get_env_var_or(key: &'static str, default: &'static str) -> String {
    std::env::var(key).unwrap_or(default.into())
}
