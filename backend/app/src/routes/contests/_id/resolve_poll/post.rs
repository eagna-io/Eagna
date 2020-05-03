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
    choice_name: ChoiceName,
}

pub fn route(ctx: Context) -> warp::filters::BoxedFilter<(Response,)> {
    warp::path!("contests" / ContestId / "resolve_poll")
        .and(warp::filters::method::post())
        .and(auth::admin())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(move |contest_id, _access_token, body| {
            handler_fn(move || inner(ctx, body, contest_id))
        })
        .recover(Error::recover)
        .unify()
        .boxed()
}

async fn inner(ctx: Context, body: ReqBody, contest_id: ContestId) -> Result<Response, Error> {
    let mut contest_repo = ContestRepository::new();

    let mut contest = contest_repo
        .query_by_id(contest_id)
        .await?
        .ok_or_else(|| Error::new(StatusCode::NOT_FOUND, "Contest not found"))?;

    contest
        .resolve_poll(account_id, body.choice_name)
        .map_err(|e| {
            log::info!("Failed to resolve poll because of {:?}", e);
            Error::new(StatusCode::BAD_REQUEST, "Failed to resolve poll")
        })?;

    contest_repo.save(contest).await?;

    Ok(response::new(StatusCode::OK, &"poll resolved"))
}
