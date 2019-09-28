use super::ResPrize;
use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::{
    prize::{Prize, PrizeRepository},
    user::{UserId, UserRepository},
};
use crate::primitive::NonEmptyString;
use rouille::{input::json_input, Request, Response};
use std::num::NonZeroU32;

pub fn post(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    // Prizeを作成する権限があるかどうかチェック
    let access_token = validate_bearer_header(infra, req)?;
    authorize(infra, &access_token.user_id)?;

    let req_prize = json_input::<ReqPrize>(req).map_err(|e| {
        log::warn!("Invalid payload error : {:?}", e);
        FailureResponse::InvalidPayload
    })?;

    let new_prize = Prize::new(
        req_prize.name,
        req_prize.description,
        req_prize.thumbnail_url,
        req_prize.price,
        req_prize.available,
    );
    let prize_repo = PrizeRepository::from(infra.get_postgres()?);
    prize_repo.save_prize(&new_prize)?;

    let res_prize = ResPrize::from(&new_prize);
    Ok(Response::json(&res_prize).with_status_code(201))
}

fn authorize(infra: &InfraManager, user_id: &UserId) -> Result<(), FailureResponse> {
    let user_repo = UserRepository::from(infra.get_postgres()?);
    match user_repo.query_user(user_id)? {
        Some(user) => {
            if user.is_admin() {
                Ok(())
            } else {
                log::warn!("Non admin user try to access admin resource");
                Err(FailureResponse::Unauthorized)
            }
        }
        None => {
            log::error!("User does not exist, but AccessToken exists");
            Err(FailureResponse::ServerError)
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReqPrize {
    name: NonEmptyString,
    description: String,
    thumbnail_url: String,
    price: NonZeroU32,
    available: bool,
}
