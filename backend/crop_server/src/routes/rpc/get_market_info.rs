use crate::context::Context;
use crop_domain::market::model::MarketId;
use crop_primitive::String as MyString;
use futures::future::FutureExt as _;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use warp::{reject::Rejection, reply::Reply, Filter};
use warp_json_rpc::{
    filters::{json_rpc, method, params},
    Builder, Error,
};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    market_id: MarketId,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct Success {
    id: MarketId,
    title: MyString,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct Failure();

pub fn filter(ctx: Context) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    json_rpc()
        .and(method("getmarketinfo"))
        .and(params::<Params>())
        .and_then(move |res: Builder, params: Params| {
            handler(params, ctx.clone()).map(|r| Ok::<_, Rejection>(res.result(r).unwrap()))
        })
}

async fn handler(params: Params, ctx: Context) -> Result<Success, Error> {
    if let Some(manager) = ctx.get_market_state(params.market_id).await {
        manager
            .with_market(|market| {
                Ok(Success {
                    id: market.id,
                    title: market.title.clone(),
                })
            })
            .await
    } else {
        Err(Error::custom(1, "Market not found"))
    }
}
