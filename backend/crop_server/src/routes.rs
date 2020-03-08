pub mod rpc;
pub mod ws;

use warp::{reject::Rejection, reply::Reply, Filter};

pub fn filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    self::rpc::filter().or(self::ws::filter())
}
