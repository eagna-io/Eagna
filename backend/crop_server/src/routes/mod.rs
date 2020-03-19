pub mod contest;
pub mod ws;

use crate::context::Context;
use warp::{filters::cors, reject::Rejection, reply::Reply, Filter};

pub fn filter(ctx: Context) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let cors_wrapper = cors::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST", "OPTIONS"])
        .allow_header("Content-Type");

    let rest = self::contest::poll::get::filter(ctx.clone())
        .or(self::contest::poll::post::filter(ctx.clone()));

    let rest_with_cors = rest.with(cors_wrapper);

    self::ws::filter(ctx).or(rest_with_cors)
}
