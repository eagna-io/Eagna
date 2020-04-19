use crate::{
    context::Context,
    error::Error,
    filters::auth,
    response::{self, Response},
};
use crop_domain::contest::poll::BriefPoll;
use crop_domain::contest::{
    BriefContest, Contest, ContestId, ContestRepository, ContestStatus, DetailedContest,
};
use http::StatusCode;
use schemars::JsonSchema;
use serde::Deserialize;
use warp::Filter as _;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReqBody {
    status: ContestStatus,
}

pub fn route(ctx: Context) -> warp::filters::BoxedFilter<(Response,)> {
    warp::path!("contests" / ContestId)
        .and(warp::filters::method::patch())
        .and(auth::admin())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(move |contest_id, _admin, body| {
            ctx.clone()
                .handle_request(move |ctx| inner(ctx, body, contest_id))
        })
        .recover(Error::recover)
        .unify()
        .boxed()
}

async fn inner(ctx: Context, body: ReqBody, contest_id: ContestId) -> Result<Response, Error> {
    match body.status {
        ContestStatus::Closed => close_contest(ctx, contest_id).await,
        ContestStatus::Archived => archive_contest(ctx, contest_id).await,
        _ => Err(Error::new(
            StatusCode::BAD_REQUEST,
            "Unsupported status change",
        )),
    }
}

async fn close_contest(ctx: Context, contest_id: ContestId) -> Result<Response, Error> {
    ctx.pg
        .with_conn::<Result<Response, Error>, _>(move |conn| {
            let contest =
                ContestRepository::query_by_id::<DetailedContest<BriefPoll>>(&conn, &contest_id)?
                    .ok_or_else(|| Error::new(StatusCode::NOT_FOUND, "Contest not found"))?;
            let closed = contest.close().map_err(|e| {
                log::info!("Failed to close contest because of {:?}", e);
                Error::new(StatusCode::BAD_REQUEST, "Failed to close contest")
            })?;

            ContestRepository::save(&conn, &closed)?;

            Ok(response::new(StatusCode::OK, &"closed"))
        })
        .await?
}

async fn archive_contest(ctx: Context, contest_id: ContestId) -> Result<Response, Error> {
    ctx.pg
        .with_conn::<Result<Response, Error>, _>(move |conn| {
            let contest = ContestRepository::query_by_id::<BriefContest>(&conn, &contest_id)?
                .ok_or_else(|| Error::new(StatusCode::NOT_FOUND, "Contest not found"))?;
            let archived = contest.archive().map_err(|e| {
                log::info!("Failed to close contest because of {:?}", e);
                Error::new(StatusCode::BAD_REQUEST, "Failed to close contest")
            })?;

            ContestRepository::save(&conn, &archived)?;

            Ok(response::new(StatusCode::OK, &"closed"))
        })
        .await?
}
