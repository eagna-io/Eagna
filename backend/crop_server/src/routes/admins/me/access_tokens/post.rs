use crate::{
    context::Context,
    error::Error,
    response::{self, Response},
};
use crop_domain::admin::{
    self,
    model::{AccessToken, Admin as _},
};
use http::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use warp::{
    filters::{body, method},
    reject::Rejection,
    Filter,
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReqBody {
    email: String,
    pass: String,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct ResBody(AccessToken);

pub fn route(ctx: Context) -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("admins" / "me" / "access_tokens")
        .and(method::post())
        .and(body::json::<ReqBody>())
        .and_then(move |body| ctx.clone().handle_request(move |ctx| inner(ctx, body)))
        .recover(Error::recover)
        .unify()
}

async fn inner(ctx: Context, body: ReqBody) -> Result<Response, Error> {
    ctx.pg
        .with_conn::<Result<Response, Error>, _>(move |conn| {
            admin::repository::query_unauthenticated(&conn, body.email.as_str())?
                .ok_or_else(|| {
                    log::info!("admin not found");
                    Error::new(StatusCode::UNAUTHORIZED, "Unauthorized")
                })?
                .authenticate(body.pass.as_str())
                .map_err(|_| {
                    log::info!("failed to auth admin");
                    Error::new(StatusCode::UNAUTHORIZED, "Unauthorized")
                })
                .map(|admin| {
                    let token = admin.gen_access_token();
                    response::new(StatusCode::CREATED, &ResBody(token))
                })
        })
        .await?
}