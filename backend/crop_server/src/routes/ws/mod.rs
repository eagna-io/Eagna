pub mod msg;
pub mod session;

use crate::context::Context;
use warp::{filters, reject::Rejection, reply::Reply, Filter};

pub fn filter(ctx: Context) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    filters::path::path("ws")
        .and(filters::path::end())
        .and(filters::ws::ws())
        .and_then(move |ws| upgrade_ws(ws, ctx.clone()))
}

async fn upgrade_ws(ws: filters::ws::Ws, ctx: Context) -> Result<impl Reply, Rejection> {
    let contest = ctx.contest_manager();
    Ok(ws.on_upgrade(move |ws| session::new(contest, ws).handle()))
}
