use crate::context::Context;
use crop_domain::admin::{
    self,
    model::{AccessToken, Admin as _},
};
use crop_infra::pg::Connection;
use http::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use warp::{
    filters::{body, method},
    reject::Rejection,
    reply::{self, Reply},
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

pub fn route(ctx: Context) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("admins" / "me" / "access_tokens")
        .and(method::post())
        .and(body::json::<ReqBody>())
        .and_then(move |body| {
            let ctx = ctx.clone();
            async move {
                let res = ctx.pg.with_conn(|conn| inner(conn, body)).await;
                let rep = match res {
                    Ok(Ok((status, res))) => reply::with_status(reply::json(&res), status),
                    Ok(Err((status, err))) => reply::with_status(reply::json(&err), status),
                    Err(e) => {
                        log::error!("{:?}", e);
                        reply::with_status(
                            reply::json(&"Internal server error"),
                            StatusCode::INTERNAL_SERVER_ERROR,
                        )
                    }
                };
                Ok::<_, Rejection>(rep)
            }
        })
}

fn inner(
    conn: Connection,
    body: ReqBody,
) -> Result<(StatusCode, ResBody), (StatusCode, &'static str)> {
    match admin::repository::query_unauthenticated(&conn, body.email.as_str()) {
        Ok(Some(unauth)) => match unauth.authenticate(body.pass.as_str()) {
            Ok(admin) => {
                let token = admin.gen_access_token();
                Ok((StatusCode::CREATED, ResBody(token)))
            }
            Err(_) => {
                log::info!("failed to auth admin");
                Err((StatusCode::UNAUTHORIZED, "Unauthorized"))
            }
        },
        Ok(None) => {
            log::info!("admin not found");
            Err((StatusCode::UNAUTHORIZED, "Unauthorized"))
        }
        Err(e) => {
            log::error!("{:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))
        }
    }
}
