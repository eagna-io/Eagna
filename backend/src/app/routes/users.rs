use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::user::*;
use rouille::{input::json::json_input, Request, Response};
use serde::{Deserialize, Serialize};

pub fn me(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let access_token = validate_bearer_header(infra, req)?;
    let repo = UserRepository::from(infra.get_postgres()?);
    if let Some(user) = repo.query_user(&access_token.user_id)? {
        let res_data = ResData {
            id: user.id(),
            name: user.name(),
            email: user.email(),
            point: repo.query_user_point(&access_token.user_id)?,
            is_admin: user.is_admin(),
        };
        return Ok(Response::json(&res_data));
    } else {
        return Err(FailureResponse::Unauthorized);
    }
}

pub fn post(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let access_token = validate_bearer_header(infra, req)?;

    let req_data = json_input::<ReqData>(req).map_err(|_| FailureResponse::InvalidPayload)?;

    let new_user = User::new(access_token.user_id, req_data.name, req_data.email);

    let user_repo = UserRepository::from(infra.get_postgres()?);

    user_repo.save_user(&new_user)?;

    let res_data = ResData {
        id: new_user.id(),
        name: new_user.name(),
        email: new_user.email(),
        point: 0,
        is_admin: new_user.is_admin(),
    };

    Ok(Response::json(&res_data).with_status_code(201))
}

#[derive(Deserialize)]
struct ReqData {
    name: UserName,
    email: UserEmail,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ResData<'a> {
    id: &'a UserId,
    name: &'a UserName,
    email: &'a UserEmail,
    point: u32,
    is_admin: bool,
}
