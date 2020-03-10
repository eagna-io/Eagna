pub mod vote;

use crate::context::Context;
use warp::{filters::path, reject::Rejection, reply::Reply, Filter};

/// ## JSON RPC
///
/// ### Path
///
/// /rpc
pub fn filter(ctx: Context) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path::path("rpc")
        .and(path::end())
        .and(self::vote::filter(ctx))
}
