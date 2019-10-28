use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::user::*;
use rouille::{Request, Response};
use serde::Serialize;

pub fn get_me(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let access_token = validate_bearer_header(infra, req)?;
    let repo = UserRepository::from(infra.get_postgres()?);
    match repo.query_user(&access_token.user_id)? {
        None => Err(FailureResponse::Unauthorized),
        Some(user) => Ok(Response::json(&construct_response_data(
            &user.with_point()?,
        ))),
    }
}

fn construct_response_data<U: UserWithPoint>(user: &U) -> ResUser {
    ResUser {
        id: user.id().as_str(),
        name: user.name().as_str(),
        email: user.email().as_str(),
        is_admin: user.is_admin(),
        point: user.point().as_u32(),
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ResUser<'a> {
    id: &'a str,
    name: &'a str,
    email: &'a str,
    is_admin: bool,
    point: u32,
}
