use crate::context::Context;
use std::convert::Infallible;
use warp::{filters::method, reject::Rejection, reply, Filter};

/// GET /contest/poll/current
///
/// ## Response
///
/// [`Poll`]
///
/// [`Poll`]: /crop_domain/poll/model/struct.Poll.html
pub fn filter(
    ctx: Context,
) -> impl Filter<Extract = (impl reply::Reply,), Error = Rejection> + Clone {
    warp::path!("contest" / "poll" / "current")
        .and(method::get())
        .and_then(move || handler(ctx.clone()))
}

async fn handler(ctx: Context) -> Result<impl reply::Reply, Infallible> {
    ctx.contest_manager()
        .with_contest(|contest, _| {
            if let Some(poll) = contest.current_poll() {
                Ok(reply::with_status(reply::json(poll), http::StatusCode::OK))
            } else {
                Ok(reply::with_status(
                    reply::json(&()),
                    http::StatusCode::NOT_FOUND,
                ))
            }
        })
        .await
}
