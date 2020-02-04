use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::market::num::AmountCoin;
use crate::domain::user::{
    models::{UserId, UserWithAttrs as _},
    repository::UserRepository,
};
use rouille::{input::json_input, Request, Response};
use uuid::Uuid;

pub fn handler(
    infra: &InfraManager,
    req: &Request,
    user_id: Uuid,
) -> Result<Response, FailureResponse> {
    let access_token = validate_bearer_header(req)?;

    let ReqData { provided_coin } = json_input(req).map_err(|_| FailureResponse::InvalidPayload)?;

    let repo = UserRepository::from(infra.get_postgres()?);
    let admin = repo
        .query_user(&access_token.user_id)?
        .ok_or(FailureResponse::Unauthorized)?
        .into_admin()
        .map_err(|_| FailureResponse::Unauthorized)?;

    let user = repo
        .query_user(&UserId::from(user_id))?
        .ok_or(FailureResponse::InvalidPayload)?;
    let provided_user = admin.provide_coin_to_user(user, AmountCoin::from(provided_coin as i32));

    repo.update_user(&provided_user)?;

    Ok(Response::json(&ResData {
        current_coin: provided_user.coin().as_i32() as u32,
    }))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReqData {
    provided_coin: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ResData {
    current_coin: u32,
}
