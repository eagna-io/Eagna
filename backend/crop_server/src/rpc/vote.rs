use serde::Deserialize;
use uuid::Uuid;
use warp::{reject::Rejection, reply::Reply, Filter};
use warp_json_rpc::{
    filters::{json_rpc, method, params},
    Builder,
};

#[derive(Debug, Deserialize)]
struct Params {
    market_id: Uuid,
}

pub fn filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    json_rpc()
        .and(method("vote"))
        .and(params::<Params>())
        .map(|res: Builder, params: Params| res.success("HOGE").unwrap())
}
