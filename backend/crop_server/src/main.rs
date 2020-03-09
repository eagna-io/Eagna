use crop_domain::market::model::{Market, OutcomeId};
use crop_server::{context::Context, server};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let context = Context::new();
    let outcome1_id = OutcomeId(Uuid::parse_str("4ef1a32161bd4c5684c1ddb327d38b91").unwrap());
    let outcome2_id = OutcomeId(Uuid::parse_str("a490323fc5444ffbbc093f18496c3e1f").unwrap());
    context
        .add_new_market(Market::new(&[outcome1_id, outcome2_id]))
        .await;

    server::Server::bind(([127, 0, 0, 1], 3030), context).await;
}