use crop_domain::market::model::Market;
use crop_server::{context::Context, server};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let context = Context::new();
    let market = Market::new("テストマーケット".into());

    println!("Market id : {}", market.id.0);

    context.add_new_market(market).await;

    server::Server::bind(([127, 0, 0, 1], 8080), context).await;
}
