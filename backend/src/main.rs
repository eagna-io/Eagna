use libeagna::app::{ApiServer, InfraManagerFactory};
use libeagna::infra::{FirebaseFactory, MockFirebaseFactory, PostgresFactory, RedisFactory};
use log::info;
use std::collections::HashMap;

fn main() {
    env_logger::init();

    // Postgresのセットアップ
    let pg_url = get_env_var_or_panic("PG_URL");
    let postgres_factory = PostgresFactory::new(pg_url);

    // Redisのセットアップ
    let redis_url = get_env_var_or_panic("REDIS_URL");
    let redis_factory = RedisFactory::new(redis_url);

    // Firebaseのセットアップ
    let firebase_api_key = get_env_var_or_panic("FIREBASE_API_KEY");
    let infra_manager_factory = match firebase_api_key.as_str() {
        // モックのFirebaseインフラを使用
        "USE_MOCK_FIREBASE" => {
            let mut test_data = HashMap::new();
            test_data.insert("test_user_access_token".into(), "test_user".into());
            test_data.insert("test_admin_access_token".into(), "test_admin".into());
            let firebase_factory = MockFirebaseFactory::new(test_data);
            InfraManagerFactory::new(firebase_factory, redis_factory, postgres_factory)
        }
        // 本番用のFirebaseインフラを使用
        _ => {
            let firebase_factory = FirebaseFactory::new(firebase_api_key);
            InfraManagerFactory::new(firebase_factory, redis_factory, postgres_factory)
        }
    };

    let bind = get_env_var_or_panic("BIND");
    let access_allow_hosts = get_env_var_or_panic("ACCESS_ALLOW_HOSTS");

    info!("Server is starting on {}", bind.as_str());

    ApiServer::new(infra_manager_factory, access_allow_hosts).run(bind.as_str());
}

fn get_env_var_or_panic(key: &'static str) -> String {
    std::env::var(key).expect(format!("{} is not specified", key).as_str())
}
