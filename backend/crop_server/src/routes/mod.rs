pub mod accounts;
pub mod admins;
pub mod contests;
pub mod ws;

use crate::context::Context;
use warp::{filters::cors, reject::Rejection, reply::Reply, Filter};

pub fn filter(ctx: Context) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let cors_wrapper = cors::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST", "PATCH", "PUT", "OPTIONS"])
        .allow_header("Content-Type");

    let rest_routes = contests::get::route(ctx.clone())
        .or(contests::post::route(ctx.clone()))
        .or(contests::_id::get::route(ctx.clone()))
        .or(contests::_id::patch::route(ctx.clone()))
        .or(contests::_id::polls::post::route(ctx.clone()))
        .or(contests::_id::polls::_id::comments::post::route(
            ctx.clone(),
        ))
        .or(contests::_id::polls::_id::my_choice::put::route(
            ctx.clone(),
        ))
        .or(contests::_id::polls::_id::patch::route(ctx.clone()))
        .or(accounts::post::route(ctx.clone()))
        .or(admins::me::access_tokens::post::route(ctx.clone()));

    let rest = rest_routes.with(cors_wrapper);

    let ws = ws::contests::_id::route(ctx.clone());

    rest.or(ws)
}
