mod rpc;
mod ws;

use crate::context::Context;
use warp::{reject::Rejection, reply::Reply, Filter};

pub fn filter(ctx: Context) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    self::rpc::filter(ctx.clone()).or(self::ws::filter(ctx))
}
