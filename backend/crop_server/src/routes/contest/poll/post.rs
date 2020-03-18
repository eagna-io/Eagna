use crate::context::Context;
use crop_domain::poll::model::{ChoiceColor, ChoiceName, Id as PollId, Poll};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::{filters::body, reject::Rejection, reply, Filter};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Body {
    choices: HashMap<ChoiceName, ChoiceColor>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct Response {
    id: PollId,
}

pub fn filter(
    ctx: Context,
) -> impl Filter<Extract = (impl reply::Reply,), Error = Rejection> + Clone {
    warp::path!("contest" / "poll")
        .and(body::json::<Body>())
        .and_then(move |Body { choices }| {
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
