use crate::{
    context::Context,
    res::{handler_fn, response, Error, Response},
};
use domain::account::{jccessToken, Account};
use http::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use warp::Filter as _;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReqBody {
    name: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ResBody {
    access_token: String,
}

pub fn route() -> warp::filters::BoxedFilter<(Response,)> {
    warp::path!("accounts")
        .and(warp::filters::method::post())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(move |body| handler_fn(move || inner(body)))
        .recover(Error::recover)
        .unify()
        .boxed()
}

async fn inner(body: ReqBody) -> Result<Response, Error> {
    let account = Account::new(body.name);
    AccountRepository::new().save(account).await?;
    let access_token = AccessToken::new(account.id).encode();
    Ok(response::new(StatusCode::OK, &ResBody { access_token }))
}
