use crop_domain::market::model::{Market, MarketId};
use crop_server::{context::Context, server};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let context = Context::new();
    let mut market = Market::new("テストマーケット".into());
    market.id = MarketId(Uuid::parse_str("4ef1a32161bd4c5684c1ddb327d38b91").unwrap());

    println!("Market id : {}", market.id.0);

    context.add_new_market(market).await;

    server::Server::bind(([0, 0, 0, 0], 8080), context).await;
}
