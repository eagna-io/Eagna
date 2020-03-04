pub mod msg;
pub mod session;

use self::session::Session;
use crate::state;
use crop_domain::market::model::MarketId;
use uuid::Uuid;
use warp::{filters, reject::Rejection, reply::Reply, Filter};

pub fn filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    filters::path::path("stream")
        .and(filters::path::param::<Uuid>())
        .and(filters::path::end())
        .and(filters::ws::ws())
        .and_then(upgrade_ws)
}

async fn upgrade_ws(market_id: Uuid, ws: filters::ws::Ws) -> Result<impl Reply, Rejection> {
    let market_id = MarketId(market_id);
    if let Some(market) = state::get_market_state(market_id).await {
        Ok(ws.on_upgrade(move |ws| Session::new(market, ws).handle()))
    } else {
        Err(warp::reject::not_found())
    }
}
