use crate::{
    context::Context,
    filters::auth,
    res::{handler_fn, response, Error, Response},
};
use domain::contest::{Contest, ContestId, ContestStatus};
use http::StatusCode;
use infra::pg::contest::ContestRepository;
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
        .and_then(move |contest_id, _admin, body| handler_fn(move || inner(ctx, body, contest_id)))
        .recover(Error::recover)
        .unify()
        .boxed()
}

async fn inner(ctx: Context, body: ReqBody, contest_id: ContestId) -> Result<Response, Error> {
    match body.status {
        ContestStatus::Open => open_contest(contest_id).await,
        ContestStatus::Closed => close_contest(ctx, contest_id).await,
        ContestStatus::Archived => archive_contest(contest_id).await,
        _ => Err(Error::new(
            StatusCode::BAD_REQUEST,
            "Unsupported status change",
        )),
    }
}

async fn open_contest(contest_id: ContestId) -> Result<Response, Error> {
    let mut contest_repo = ContestRepository::new();

    let mut contest = contest_repo
        .query_by_id(contest_id)
        .await?
        .ok_or_else(|| Error::new(StatusCode::NOT_FOUND, "Contest not found"))?;

    contest.open().map_err(|e| {
        log::info!("Failed to open contest because of {:?}", e);
        Error::new(StatusCode::BAD_REQUEST, "Failed to open contest")
    })?;

    contest_repo.save(contest).await?;

    Ok(response::new(StatusCode::OK, &"open"))
}

// TODO
// 結果送信
async fn close_contest(_ctx: Context, contest_id: ContestId) -> Result<Response, Error> {
    let mut contest_repo = ContestRepository::new();

    let mut contest = contest_repo
        .query_by_id(contest_id)
        .await?
        .ok_or_else(|| Error::new(StatusCode::NOT_FOUND, "Contest not found"))?;

    contest.close().map_err(|e| {
        log::info!("Failed to close contest because of {:?}", e);
        Error::new(StatusCode::BAD_REQUEST, "Failed to close contest")
    })?;

    contest_repo.save(contest).await?;

    Ok(response::new(StatusCode::OK, &"closed"))
}

async fn archive_contest(contest_id: ContestId) -> Result<Response, Error> {
    let mut contest_repo = ContestRepository::new();

    let mut contest = contest_repo
        .query_by_id(contest_id)
        .await?
        .ok_or_else(|| Error::new(StatusCode::NOT_FOUND, "Contest not found"))?;

    contest.archive().map_err(|e| {
        log::info!("Failed to close contest because of {:?}", e);
        Error::new(StatusCode::BAD_REQUEST, "Failed to close contest")
    })?;

    contest_repo.save(archived)?;

    Ok(response::new(StatusCode::OK, &"closed"))
}
