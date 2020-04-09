use crate::{context::Context, routes::ws::msg::OutgoingMsg};
use crop_domain::contest::poll::model::ChoiceName;
use schemars::JsonSchema;
use serde::Deserialize;
use std::convert::Infallible;
use warp::{
    filters::{body, method},
    reject::Rejection,
    reply, Filter,
};

/// ```json
/// {
///     "resolved": "HOGE"
/// }
/// ```
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Body {
    Resolve { resolved: ChoiceName },
}

/// PATCH /contest/poll/current
///
/// ## Request
///
/// [`Body`]
///
/// [`Body`]: ./struct.Body.html
///
/// ## Response
///
/// Empty
pub fn filter(
    ctx: Context,
) -> impl Filter<Extract = (impl reply::Reply,), Error = Rejection> + Clone {
    warp::path!("contest" / "poll")
        .and(method::patch())
        .and(body::json::<Body>())
        .and_then(move |body| {
            let ctx = ctx.clone();
            handler(ctx, body)
        })
}

async fn handler(ctx: Context, body: Body) -> Result<impl reply::Reply, Infallible> {
    let Body::Resolve { resolved } = body;
    ctx.contest_manager()
        .with_contest(|contest, sender| {
            if let Some(poll) = contest.current_poll_mut() {
                if let Err(_) = poll.resolve(resolved) {
                    return Ok(reply::with_status(
                        reply::json(&"Already resolved"),
                        http::StatusCode::BAD_REQUEST,
                    ));
                }
                let msg = OutgoingMsg::from(&*poll).into();
                let _ = sender.send(msg);
                Ok(reply::with_status(reply::json(&()), http::StatusCode::OK))
            } else {
                Ok(reply::with_status(
                    reply::json(&"No available Poll"),
                    http::StatusCode::NOT_FOUND,
                ))
            }
        })
        .await
}
