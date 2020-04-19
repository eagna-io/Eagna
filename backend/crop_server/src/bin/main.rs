use crop_infra::{jwt, pg::Pool};
use crop_server::{context::Context, server};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // jwt secretの初期化
    // TODO
    jwt::init(b"HOGEHOGE");

    // Contextの初期化
    let pg_pool = Pool::new(get_env_var_or_panic("DATABASE_URL"));
    let context = Context::new(pg_pool);

    // Serverの起動
    let port = get_env_var_u16_or_panic("PORT");
    log::info!("Server is running on port {}", port);
    server::Server::bind(([0, 0, 0, 0], port), context).await;
}

fn get_env_var_or_panic(key: &'static str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!(format!("{} is not specified", key)))
}

fn get_env_var_u16_or_panic(key: &'static str) -> u16 {
    let s = get_env_var_or_panic(key);
    u16::from_str_radix(s.as_str(), 10).unwrap()
}
