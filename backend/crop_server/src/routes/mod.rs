pub mod admins;
pub mod contests;

use crate::context::Context;
use warp::{filters::cors, reject::Rejection, reply::Reply, Filter};

pub fn filter(ctx: Context) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let cors_wrapper = cors::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST", "PATCH", "PUT", "OPTIONS"])
        .allow_header("Content-Type");

    let routes = contests::get::route(ctx.clone())
        .or(contests::post::route(ctx.clone()))
        .or(contests::_id::polls::post::route(ctx.clone()))
        .or(contests::_id::polls::_id::patch::route(ctx.clone()))
        .or(admins::me::access_tokens::post::route(ctx));

    routes.with(cors_wrapper)
}
