use crate::context::Context;
use crop_domain::poll::model::{ChoiceColor, ChoiceName, Id as PollId, Poll, Status as PollStatus};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{
    filters::{body, method},
    reject::Rejection,
    reply, Filter,
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Body {
    pub status: PollStatus,
    pub resolved: Option<ChoiceName>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct Response {
    pub id: PollId,
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
/// [`Response`]
///
/// [`Response`]: ./struct.Response.html
pub fn filter(
    ctx: Context,
) -> impl Filter<Extract = (impl reply::Reply,), Error = Rejection> + Clone {
    warp::path!("contest" / "poll")
        .and(method::post())
        .and(body::json::<Body>())
        .and_then(move |Body { status }| {
            let ctx = ctx.clone();
            async move {
                let poll = Poll::new(choices);
                let id = poll.id;
                ctx.contest_manager().add_poll_and_broadcast(poll).await;
                Ok::<_, Rejection>(reply::with_status(
                    reply::json(&Response { id }),
                    http::StatusCode::CREATED,
                ))
            }
        })
}

async fn handle_resolve(ctx: Context, choice: ChoiceName) -> Result<impl reply::Reply, Infallible> {
    match ctx.resolve_and_broadcast(choice).await {
        Ok(()) => Ok(reply::with_status(reply::json(&()), http::StatusCode::OK)),
        Err(e) => Ok(reply::with_status(
            reply::json(&e.into_string()),
            http::StatusCode::NOT_FOUND,
        )),
    }
}
