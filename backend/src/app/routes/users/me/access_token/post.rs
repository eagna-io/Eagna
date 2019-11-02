use crate::app::{FailureResponse, InfraManager};
use crate::domain::user::{
    models::User, repository::access_token::AccessTokenRepository, services::auth::UserAuthService,
};
use rouille::{input::json_input, Request, Response};

pub fn handler(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let ReqData { email, password } = json_input::<ReqData>(req).map_err(|e| {
        log::warn!("Client sends invalid format payload : {:?}", e);
        FailureResponse::InvalidPayload
    })?;

    let user = UserAuthService::from(infra.get_postgres()?)
        .authenticate(email.as_str(), password.as_str())?;
    let access_token = user.new_access_token();

    AccessTokenRepository::from(infra.get_redis()?).save(&access_token)?;

    Ok(Response::json(&ResData {
        token: access_token.id.as_str(),
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
