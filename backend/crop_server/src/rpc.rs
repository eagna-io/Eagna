mod vote;

use warp::{filters::path, reject::Rejection, reply::Reply, Filter};

pub fn filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    path::path("rpc").and(self::vote::filter())
}
