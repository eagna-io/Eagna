use crate::state;
use crop_domain::market::model::MarketId;
use futures::future::FutureExt as _;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{reject::Rejection, reply::Reply, Filter};
use warp_json_rpc::{
    filters::{json_rpc, method, params},
    Builder, Error,
};

#[derive(Debug, Deserialize)]
struct Params {
    market_id: Uuid,
}

#[derive(Debug, Serialize)]
struct Success();

#[derive(Debug, Serialize)]
struct Failure();

pub fn filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    json_rpc()
        .and(method("vote"))
        .and(params::<Params>())
        .and_then(|res: Builder, params: Params| {
            handler(params).map(|r| Ok::<_, Rejection>(res.result(r).unwrap()))
        })
}

async fn handler(params: Params) -> Result<Success, Error> {
    let market_id = MarketId(params.market_id);
    if let Some(market) = state::get_market_state(market_id).await {
        todo!()
    } else {
        todo!()
    }
}
