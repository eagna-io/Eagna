use crate::context::Context;
use crop_domain::poll::model::{ChoiceColor, ChoiceName, Id as PollId, Poll};
use futures::future::FutureExt as _;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::{reject::Rejection, reply::Reply, Filter};
use warp_json_rpc::{
    filters::{json_rpc, method, params},
    Builder, Error,
};

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    choices: HashMap<ChoiceName, ChoiceColor>,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct Success(PollId);

pub fn filter(ctx: Context) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    json_rpc()
        .and(method("createpoll"))
        .and(params::<Params>())
        .and_then(move |res: Builder, params: Params| {
            handler(params, ctx.clone()).map(|r| Ok::<_, Rejection>(res.result(r).unwrap()))
        })
}

async fn handler(params: Params, ctx: Context) -> Result<Success, Error> {
    let Params { choices } = params;
    let poll = Poll::new(choices);
    let id = poll.id;
    ctx.contest_manager().add_poll_and_broadcast(poll).await;
    Ok(Success(id))
}
