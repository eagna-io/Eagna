use crop_domain::contest::model::Contest;
use crop_server::{context::Context, server};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let context = Context::new(Contest::new());

    server::Server::bind(([0, 0, 0, 0], 8080), context).await;
}
