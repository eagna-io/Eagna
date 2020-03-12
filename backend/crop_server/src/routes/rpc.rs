pub mod get_market_info;
pub mod vote;

use crate::context::Context;
use warp::{
    filters::{cors, path},
    reject::Rejection,
    reply::Reply,
    Filter,
};

/// ## JSON RPC
///
/// ### Path
///
/// /rpc
pub fn filter(ctx: Context) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let cors = cors::cors().allow_any_origin().allow_methods(vec!["POST"]);
    path::path("rpc")
        .and(path::end())
        .and(self::vote::filter(ctx.clone()).or(self::get_market_info::filter(ctx)))
        .with(cors)
}
