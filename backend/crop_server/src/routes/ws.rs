pub mod msg;
pub mod session;

use self::session::Session;
use crate::context::Context;
use crop_domain::market::model::MarketId;
use uuid::Uuid;
use warp::{filters, reject::Rejection, reply::Reply, Filter};

pub fn filter(ctx: Context) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    filters::path::path("stream")
        .and(filters::path::param::<Uuid>())
        .and(filters::path::end())
        .and(filters::ws::ws())
        .and_then(move |market_id, ws| upgrade_ws(market_id, ws, ctx.clone()))
}

async fn upgrade_ws(
    market_id: Uuid,
    ws: filters::ws::Ws,
    ctx: Context,
) -> Result<impl Reply, Rejection> {
    let market_id = MarketId(market_id);
    if let Some(market) = ctx.get_market_state(market_id).await {
        Ok(ws.on_upgrade(move |ws| Session::new(market, ws).handle()))
    } else {
        Err(warp::reject::not_found())
    }
}
