use crate::{
    context::Context,
    error::Error,
    response::{self, Response},
};
use crop_domain::contest::poll::BriefPoll;
use crop_domain::contest::{ContestId, ContestRepository as _, DetailedContest};
use http::StatusCode;
use schemars::JsonSchema;
use serde::Serialize;
use warp::{reject::Rejection, Filter};

#[derive(Debug, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct ResBody(DetailedContest<BriefPoll>);

pub fn route(ctx: Context) -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("contests" / ContestId)
        .and(warp::filters::method::get())
        .and_then(move |contest_id| {
            ctx.clone()
                .handle_request(move |ctx| inner(contest_id, ctx))
        })
        .recover(Error::recover)
        .unify()
}

async fn inner(contest_id: ContestId, ctx: Context) -> Result<Response, Error> {
    ctx.pg
        .with_conn::<Result<Response, Error>, _>(move |conn| {
            conn.query_by_id::<DetailedContest<BriefPoll>>(&contest_id)?
                .ok_or_else(|| Error::new(StatusCode::NOT_FOUND, "Contest not found"))
                .map(|contest| response::new(StatusCode::OK, &ResBody(contest)))
        })
        .await?
}
