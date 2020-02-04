use super::ApiMarketStatus;
use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::market::{
    models::MarketId, repository::MarketRepository, services::manager::MarketManager,
};
use crate::domain::user::{models::UserId, repository::UserRepository};
use crate::infra::postgres::transaction;
use crate::primitive::NonEmptyString;

use rouille::{input::json::json_input, Request, Response};
use uuid::Uuid;

pub fn put(
    infra: &InfraManager,
    req: &Request,
    market_id: Uuid,
) -> Result<Response, FailureResponse> {
    let access_token = validate_bearer_header(req)?;

    let postgres = infra.get_postgres()?;
    transaction(postgres, || {
        let user_repo = UserRepository::from(postgres);
        authorize(&user_repo, &access_token.user_id)?;

        let req_data = json_input::<ReqPutMarket>(req).map_err(|e| {
            log::warn!("Invalid payload : {:?}", e);
            FailureResponse::InvalidPayload
        })?;

        if req_data.status != ApiMarketStatus::Resolved {
            log::warn!("Only resolving operation is supported");
            return Err(FailureResponse::InvalidPayload);
        }

        if req_data.resolved_token_name.is_none() {
            log::warn!("resolved_token_name is not set");
            return Err(FailureResponse::InvalidPayload);
        }
        let resolved_token_name = req_data.resolved_token_name.unwrap();

        let market_repo = MarketRepository::from(postgres);
        market_repo.lock_market(&MarketId::from(market_id))?;

        let closed_market = match market_repo.query_market(&MarketId::from(market_id))? {
            None => return Err(FailureResponse::ResourceNotFound),
            Some(m) => match m.into_closed_market() {
                Some(closed_market) => closed_market,
                None => {
                    log::warn!("specified market is not closed.");
                    return Err(FailureResponse::ResourceNotFound);
                }
            },
        };

        let (resolved_market, updated_users) =
            MarketManager::resolve(closed_market, resolved_token_name).map_err(|e| {
                log::warn!("{:?}", e);
                FailureResponse::InvalidPayload
            })?;

        market_repo.update_market(&resolved_market)?;
        for updated_user in updated_users {
            user_repo.update_user(&updated_user)?;
        }

        Ok(())
    })?;

    Ok(Response::json(&ResPutMarket { id: &market_id }).with_status_code(201))
}

// マーケットを作成する権限があるかチェック
fn authorize(user_repo: &UserRepository, user_id: &UserId) -> Result<(), FailureResponse> {
    match user_repo.query_user(user_id)? {
        Some(user) => {
            if user.is_admin() {
                Ok(())
            } else {
                Err(FailureResponse::Unauthorized)
            }
        }
        None => {
            log::error!("User does not exists, but AccessToken exists");
            Err(FailureResponse::ServerError)
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReqPutMarket {
    status: ApiMarketStatus,
    resolved_token_name: Option<NonEmptyString>,
}

#[derive(Debug, Serialize)]
struct ResPutMarket<'a> {
    id: &'a Uuid,
}
