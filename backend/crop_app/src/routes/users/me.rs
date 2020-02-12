pub mod access_token;
pub mod post;

use crate::{validate_bearer_header, FailureResponse, InfraManager};
use crop_domain::user::{models::UserWithAttrs, repository::UserRepository};
use rouille::{Request, Response};
use serde::Serialize;
use uuid::Uuid;

pub fn get(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let access_token = validate_bearer_header(req)?;
    let repo = UserRepository::from(infra.get_postgres()?);
    let user = match repo.query_user(&access_token.user_id)? {
        None => return Err(FailureResponse::Unauthorized),
        Some(user) => user,
    };
    Ok(Response::json(&ResUser::from(&user)))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ResUser<'a> {
    id: Uuid,
    name: &'a str,
    email: &'a str,
    is_admin: bool,
    coin: u32,
    point: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ResUserMarketRewardRecord {
    market_id: Uuid,
    point: u32,
}

impl<'a, U> From<&'a U> for ResUser<'a>
where
    U: UserWithAttrs,
{
    fn from(user: &'a U) -> ResUser<'a> {
        ResUser {
            id: *user.id().as_uuid(),
            name: user.name().as_str(),
            email: user.email().as_str(),
            is_admin: user.is_admin(),
            coin: user.coin().as_i32() as u32,
            point: user.point().as_u32(),
        }
    }
}
