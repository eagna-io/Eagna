use crate::{
    context::Context,
    filters::auth,
    res::{handler_fn, response, Error, Response},
};
use chrono::{DateTime, Utc};
use domain::{account::AccountId, comment::Comment, contest::ContestId};
use http::StatusCode;
use infra::pg::{comment::CommentRepository, contest::ContestRepository};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use warp::Filter as _;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReqBody {
    contest_id: ContestId,
    comment: String,
}

pub fn route(ctx: Context) -> warp::filters::BoxedFilter<(Response,)> {
    warp::path!("comments")
        .and(warp::filters::method::post())
        .and(auth::account())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(move |token, body| handler_fn(move || inner(ctx, body, token.account_id)))
        .recover(Error::recover)
        .unify()
        .boxed()
}

async fn inner(ctx: Context, body: ReqBody, account_id: AccountId) -> Result<Response, Error> {
    let contest = ContestRepository::new()
        .find_by_id(body.contest_id)
        .await?
        .ok_or_else(|| Error::new(StatusCode::NOT_FOUND, "Contest not found"));

    let comment = Comment::new(&contest, account_id, body.comment).map_err(|e| {
        log::info!("Failed to create comment because of {:?}", e);
        Error::new(StatusCode::BAD_REQUEST, "Failed to create comment")
    });

    CommentRepository::new().save(comment).await?;

    // TODO
    // 作成したコメントのブロードキャスト

    Ok(response::new(StatusCode::CREATED, &"Created"))
}
