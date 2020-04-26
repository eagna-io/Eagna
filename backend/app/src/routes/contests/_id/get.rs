use crate::{
    context::Context,
    res::{response, Error, Response},
};
use domain::contest::ContestId;
use http::StatusCode;
use infra::pg::contest::ContestRepository;
use schemars::JsonSchema;
use serde::Serialize;
use warp::Filter as _;

#[derive(Debug, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct ResBody(Contest);

pub fn route() -> warp::filters::BoxedFilter<(Response,)> {
    warp::path!("contests" / ContestId)
        .and(warp::filters::method::get())
        .and_then(move |contest_id| handler_fn(move || inner(contest_id)))
        .recover(Error::recover)
        .unify()
        .boxed()
}

async fn inner(contest_id: ContestId) -> Result<Response, Error> {
    ContestRepository::new()
        .find_by_id(contest_id)
        .await?
        .ok_or_else(|| Error::new(StatusCode::NOT_FOUND, "Contest not found"))
        .map(|contest| response(StatusCode::OK, &ResBody(contest)))
}
