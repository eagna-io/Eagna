use crate::{FailureResponse, InfraManager};
use crop_domain::user::access_token::services::AccessTokenManager;
use crop_domain::user::{models::User, services::auth::UserAuthService};
use rouille::{input::json_input, Request, Response};

pub fn handler(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let ReqData { email, password } = json_input::<ReqData>(req).map_err(|e| {
        log::warn!("Client sends invalid format payload : {:?}", e);
        FailureResponse::InvalidPayload
    })?;

    let user = UserAuthService::from(infra.get_postgres()?)
        .authenticate(email.as_str(), password.as_str())
        .map_err(|_| FailureResponse::Unauthorized)?;
    let access_token = user.new_access_token();
    let encoded_token = AccessTokenManager::encode(&access_token);

    Ok(Response::json(&ResData {
        token: encoded_token.as_str(),
    })
    .with_status_code(201))
}

#[derive(Deserialize)]
struct ReqData {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct ResData<'a> {
    token: &'a str,
}
