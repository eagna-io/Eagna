pub mod create_poll;

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
    let cors = cors::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST", "OPTIONS"])
        .allow_header("Content-Type");
    path::path("rpc")
        .and(path::end())
        .and(self::create_poll::filter(ctx.clone()))
        .with(cors)
}
