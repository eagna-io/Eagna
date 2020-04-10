use crate::{
    context::Context,
    error::Error,
    response::{self, Response},
};
use crop_domain::contest::{
    model::{Contest, Unknown},
    repository::ContestRepository as _,
};
use http::StatusCode;
use schemars::JsonSchema;
use serde::Serialize;
use warp::{reject::Rejection, Filter};

#[derive(Debug, Serialize, JsonSchema)]
pub struct ResBody {
    contests: Vec<Contest<Unknown>>,
}

pub fn route(ctx: Context) -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("contests")
        .and(warp::filters::method::get())
        .and_then(move || ctx.clone().handle_request(inner))
        .recover(Error::recover)
        .unify()
}

async fn inner(ctx: Context) -> Result<Response, Error> {
    ctx.pg
        .with_conn::<Result<Response, Error>, _>(|conn| {
            let body = ResBody {
                contests: conn.query_not_archived()?,
            };
            Ok(response::new(StatusCode::OK, &body))
        })
        .await?
}
