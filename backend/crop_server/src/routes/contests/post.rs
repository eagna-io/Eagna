use crate::{
    context::Context,
    error::Error,
    filters::auth,
    response::{self, Response},
};
use chrono::{DateTime, Utc};
use crop_domain::contest::{
    self,
    model::{Contest, ContestId},
    repository::ContestRepository as _,
};
use http::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use warp::{reject::Rejection, Filter};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReqBody {
    title: String,
    category: String,
    event_start_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct ResBody(ContestId);

pub fn route(ctx: Context) -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("contests")
        .and(warp::filters::method::post())
        .and(auth::admin())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(move |_admin, body| ctx.clone().handle_request(move |ctx| inner(ctx, body)))
        .recover(Error::recover)
        .unify()
}

async fn inner(ctx: Context, body: ReqBody) -> Result<Response, Error> {
    let contest_id = ctx
        .pg
        .with_conn::<Result<ContestId, Error>, _>(move |conn| {
            let contest = contest::new(body.title, body.category, body.event_start_at);
            conn.save(&contest)?;
            Ok(contest.id())
        })
        .await??;

    ctx.contest_manager.enable_subscribe(contest_id).await;

    Ok(response::new(StatusCode::CREATED, &ResBody(contest_id)))
}
