use crate::{
    context::Context,
    error::Error,
    response::{self, Response},
};
use crop_domain::account::{self, AccessToken, Account, AccountRepository};
use http::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use warp::{reject::Rejection, Filter};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReqBody {
    name: String,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct ResBody(AccessToken);

pub fn route(ctx: Context) -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("accounts")
        .and(warp::filters::method::post())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(move |body| ctx.clone().handle_request(move |ctx| inner(ctx, body)))
        .recover(Error::recover)
        .unify()
}

async fn inner(ctx: Context, body: ReqBody) -> Result<Response, Error> {
    ctx.pg
        .with_conn::<Result<Response, Error>, _>(move |conn| {
            let account = account::new(body.name);
            conn.save(&account)?;
            let token = account.gen_access_token();
            Ok(response::new(StatusCode::OK, &ResBody(token)))
        })
        .await?
}
