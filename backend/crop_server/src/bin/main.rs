use crop_domain::contest::model::Contest;
use crop_server::{context::Context, server};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let context = Context::new(Contest::new());

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
