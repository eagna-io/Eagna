use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::user::*;
use rouille::{input::json::json_input, Request, Response};

pub fn post(infra: InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let req_data = json_input::<ReqData>(req).map_err(|_| FailureResponse::InvalidPayload)?;

    let access_token = validate_bearer_header(&infra, req)?;

    let new_user = User::new(access_token.user_id, req_data.name, req_data.email);

    let user_repo = UserRepository::from(infra.get_postgres()?);

    user_repo.save_user(new_user)?;

    let res_data = ResData {
        id: new_user.id(),
        name: new_user.name(),
        email: new_user.email(),
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
    is_admin: bool,
}
