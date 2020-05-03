use crate::{
    context::Context,
    res::{handler_fn, response, Error, Response},
};
use domain::admin::Admin;
use http::StatusCode;
use infra::pg::admin::AdminRepository;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use warp::Filter as _;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReqBody {
    email: String,
    pass: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ResBody {
    access_token: String,
}

pub fn route() -> warp::filters::BoxedFilter<(Response,)> {
    warp::path!("admins" / "me" / "access_tokens")
        .and(warp::filters::method::post())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(move |body| handler_fn(move || inner(body)))
        .recover(Error::recover)
        .unify()
        .boxed()
}

async fn inner(body: ReqBody) -> Result<Response, Error> {
    AdminRepository::new()
        .query_unauthenticated(body.email.as_str())
        .await?
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
            let access_token = AccessToken::new(&admin).encode();
            response::new(StatusCode::CREATED, &ResBody { access_token })
        })
}
