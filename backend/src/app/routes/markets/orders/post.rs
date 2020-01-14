use super::{ReqOrder, ResOrder};
use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::market::{
    models::{MarketId, OpenMarket},
    num::{AmountCoin, AmountToken},
    repository::MarketRepository,
    services::manager::{MarketManager, OpenMarketOrderAdded},
};
use crate::domain::user::{
    models::{UserCoinUpdated, UserWithAttrs},
    repository::UserRepository,
};
use crate::infra::postgres::transaction;

use rouille::{input::json::json_input, Request, Response};
use uuid::Uuid;

pub fn post(
    infra: &InfraManager,
    req: &Request,
    market_id: Uuid,
) -> Result<Response, FailureResponse> {
    let req_data = json_input::<ReqOrder>(req).map_err(|e| {
        log::warn!("Receive invalid payload request : {:?}", e);
        FailureResponse::InvalidPayload
    })?;

    validate_req_order(&req_data)?;

    let user_id = validate_bearer_header(infra, req)?.user_id;
    let postgres = infra.get_postgres()?;
    let user = UserRepository::from(postgres)
        .query_user(&user_id)?
        .ok_or(FailureResponse::InvalidPayload)?;

    transaction(postgres, || {
        let market_repo = MarketRepository::from(postgres);

        market_repo.lock_market(&MarketId::from(market_id))?;

        let open_market = match market_repo.query_market(&MarketId::from(market_id))? {
            Some(m) => match m.into_open_market() {
                Some(open_market) => open_market,
                None => {
                    log::warn!("User requests order for not opened market : ${}", market_id);
                    return Err(FailureResponse::ResourceNotFound);
                }
            },
            None => {
                log::warn!("User requests order for not exist market : ${}", market_id);
                return Err(FailureResponse::ResourceNotFound);
            }
        };

        let (updated_market, updated_user) = add_order(open_market, user, &req_data)?;

        market_repo.update_market(&updated_market)?;
        UserRepository::from(postgres).update_user(&updated_user)?;

        let res_data = ResOrder::from(updated_market.added_order());

        Ok(Response::json(&res_data).with_status_code(201))
    })
}

fn validate_req_order(req_order: &ReqOrder) -> Result<(), FailureResponse> {
    if req_order.amount_token == 0 || req_order.amount_coin == 0 {
        log::warn!("Received 0 amount order request {:?}", req_order);
        Err(FailureResponse::InvalidPayload)
    } else {
        Ok(())
    }
}

fn add_order<M, U>(
    market: M,
    user: U,
    req: &ReqOrder,
) -> Result<(OpenMarketOrderAdded<M>, UserCoinUpdated<U>), FailureResponse>
where
    M: OpenMarket,
    U: UserWithAttrs,
{
    if req.amount_token > 0 {
        // buy
        Ok(MarketManager::buy_token(
            market,
            user,
            &req.token_name,
            &AmountToken::from(req.amount_token),
            &AmountCoin::from(-req.amount_coin),
        )
        .map_err(|e| e.source)?)
    } else {
        // sell
        Ok(MarketManager::sell_token(
            market,
            user,
            &req.token_name,
            &AmountToken::from(-req.amount_token),
            &AmountCoin::from(req.amount_coin),
        )
        .map_err(|e| e.source)?)
    }
}
