use crate::context::Context;
use crop_domain::{
    account::model::AccountId,
    market::model::{MarketId, OutcomeId},
};
use futures::future::FutureExt as _;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{reject::Rejection, reply::Reply, Filter};
use warp_json_rpc::{
    filters::{json_rpc, method, params},
    Builder, Error,
};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    market_id: Uuid,
    account_id: Uuid,
    outcome_id: Uuid,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct Success();

#[derive(Debug, Serialize, JsonSchema)]
pub struct Failure();

pub fn filter(ctx: Context) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    json_rpc()
        .and(method("vote"))
        .and(params::<Params>())
        .and_then(move |res: Builder, params: Params| {
            handler(params, ctx.clone()).map(|r| Ok::<_, Rejection>(res.result(r).unwrap()))
        })
}

async fn handler(params: Params, ctx: Context) -> Result<Success, Error> {
    let market_id = MarketId(params.market_id);
    if let Some(market) = ctx.get_market_state(market_id).await {
        // 現在はこのorderを特に使っていない
        let account_id = AccountId(params.account_id);
        let outcome_id = OutcomeId(params.outcome_id);
        let _order = market.vote_and_broadcast(account_id, outcome_id).await;
        Ok(Success())
    } else {
        Err(Error::custom(1, "Market not found"))
    }
}
