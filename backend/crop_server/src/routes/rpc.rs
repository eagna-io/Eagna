mod vote;

use warp::{filters::path, reject::Rejection, reply::Reply, Filter};

/// ## JSON RPC
///
/// ### Path
///
/// /rpc
pub fn filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    path::path("rpc").and(path::end()).and(self::vote::filter())
}
