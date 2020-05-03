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
    warp::path!("contests" / ContestId / "update_answer")
        .and(warp::filters::method::post())
        .and(auth::account())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(move |contest_id, access_token, body| {
            handler_fn(move || inner(ctx, body, contest_id, access_token.account_id))
        })
        .recover(Error::recover)
        .unify()
        .boxed()
}

async fn inner(
    ctx: Context,
    body: ReqBody,
    contest_id: ContestId,
    account_id: AccountId,
) -> Result<Response, Error> {
    let mut contest_repo = ContestRepository::new();

    let mut contest = contest_repo
        .query_by_id(contest_id)
        .await?
        .ok_or_else(|| Error::new(StatusCode::NOT_FOUND, "Contest not found"))?;

    contest
        .update_answer(account_id, body.choice_name)
        .map_err(|e| {
            log::info!("Failed to update answer because of {:?}", e);
            Error::new(StatusCode::BAD_REQUEST, "Failed to update answer")
        })?;

    contest_repo.save(contest).await?;

    Ok(response::new(StatusCode::OK, &"updated"))
}
