use super::ApiMarketStatus;
use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::{market::services::resolver::resolve_market_uncheck, market::*, user::*};
use crate::infra::postgres::{transaction, PostgresInfra};
use crate::primitive::NonEmptyString;

use rouille::{input::json::json_input, Request, Response};
use uuid::Uuid;

pub fn put(
    infra: &InfraManager,
    req: &Request,
    market_id: Uuid,
) -> Result<Response, FailureResponse> {
    let access_token = validate_bearer_header(infra, req)?;

    let postgres = infra.get_postgres()?;
    transaction(postgres, || {
        authorize(postgres, &access_token.user_id)?;

        let req_data = json_input::<ReqPutMarket>(req).map_err(|e| {
            log::warn!("Invalid payload : {:?}", e);
            FailureResponse::InvalidPayload
        })?;

        if req_data.status != ApiMarketStatus::Resolved {
            log::info!("Only resolving operation is supported");
            return Err(FailureResponse::InvalidPayload);
        }

        if req_data.resolved_token_name.is_none() {
            log::info!("resolved_token_name is not set");
            return Err(FailureResponse::InvalidPayload);
        }
        let resolved_token_name = req_data.resolved_token_name.unwrap();

        let market_repo = MarketRepository::from(postgres);
        market_repo.lock_market(&MarketId::from(market_id))?;

        let closed_market = match market_repo.query_market(&MarketId::from(market_id))? {
            Some(Market::Closed(m)) => m,
            Some(_) => {
                log::warn!("specified market is not closed.");
                return Err(FailureResponse::ResourceNotFound);
            }
            None => return Err(FailureResponse::ResourceNotFound),
        };

        if !closed_market.attrs().is_valid_token(&resolved_token_name) {
            log::warn!("invalid resolved token : {:?}", resolved_token_name);
            return Err(FailureResponse::InvalidPayload);
        }
        let resolved_market = resolve_market_uncheck(closed_market, resolved_token_name);

        market_repo.save_market(&Market::from(resolved_market))?;

        Ok(())
    })?;

    Ok(Response::json(&ResPutMarket { id: &market_id }).with_status_code(201))
}

// マーケットを作成する権限があるかチェック
fn authorize(postgres: &dyn PostgresInfra, user_id: &UserId) -> Result<(), FailureResponse> {
    let user_repo = UserRepository::from(postgres);

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
