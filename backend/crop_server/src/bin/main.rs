use crop_domain::contest::model::Contest;
use crop_server::{context::Context, server};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let context = Context::new(Contest::new());

    log::info!("Server is running on port {}", 8080);

    server::Server::bind(([127, 0, 0, 1], 8080), context).await;
}
