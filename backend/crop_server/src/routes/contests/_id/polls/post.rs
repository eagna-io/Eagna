use crate::{
    context::Context,
    error::Error,
    filters::auth,
    response::{self, Response},
};
use chrono::Duration;
use crop_domain::contest::poll::{ChoiceColor, ChoiceName, Poll, PollId};
use crop_domain::contest::{BriefContest, Contest, ContestId, ContestRepository as _};
use http::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::{reject::Rejection, Filter};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReqBody {
    title: String,
    duration_sec: Option<i32>,
    choices: HashMap<ChoiceName, ChoiceColor>,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct ResBody(PollId);

pub fn route(ctx: Context) -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("contests" / ContestId / "polls")
        .and(warp::filters::method::post())
        .and(auth::admin())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(move |contest_id, _admin, body| {
            ctx.clone()
                .handle_request(move |ctx| inner(ctx, contest_id, body))
        })
        .recover(Error::recover)
        .unify()
}

async fn inner(ctx: Context, contest_id: ContestId, body: ReqBody) -> Result<Response, Error> {
    ctx.pg
        .with_conn::<Result<Response, Error>, _>(move |conn| {
            let duration = body.duration_sec.map(|s| Duration::seconds(s as i64));
            let contest = conn
                .query_by_id::<BriefContest>(&contest_id)?
                .ok_or(Error::new(StatusCode::NOT_FOUND, "Contest not found"))?;
            let added = contest.add_poll(body.title, duration, body.choices)?;
            conn.save(&added)?;
            Ok(response::new(
                StatusCode::CREATED,
                &ResBody(added.poll().id()),
            ))
        })
        .await?
}
