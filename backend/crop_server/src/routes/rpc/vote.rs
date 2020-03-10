use crate::context::Context;
use crop_domain::{
    account::model::AccountName,
    market::model::{MarketId, OutcomeId},
};
use crop_primitive::String as MyString;
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
    account_name: MyString,
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
        let account_name = AccountName(params.account_name);
        let outcome_id = OutcomeId(params.outcome_id);
        // 現在はこのorderを特に使っていない
        let _order = market.vote_and_broadcast(account_name, outcome_id).await;
        Ok(Success())
    } else {
        Err(Error::custom(1, "Market not found"))
    }
}
