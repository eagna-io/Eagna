use super::{ApiOrderType, ReqOrder, ResOrder};
use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::{market::*, user::*};
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
    let added_order = transaction(postgres, || {
        let market_repo = MarketRepository::from(postgres);

        market_repo.lock_market(&MarketId::from(market_id))?;

        let mut open_market = match market_repo.query_market(&MarketId::from(market_id))? {
            Some(Market::Open(m)) => m,
            Some(_) => {
                log::warn!("User requests order for not opened market : ${}", market_id);
                return Err(FailureResponse::ResourceNotFound);
            }
            None => {
                log::warn!("User requests order for not exist market : ${}", market_id);
                return Err(FailureResponse::ResourceNotFound);
            }
        };

        let added_order = add_order(&mut open_market, &user_id, &req_data)?;

        market_repo.save_market(&Market::from(open_market))?;

        Ok(added_order)
    })?;

    let res_data = ResOrder::from(&added_order);

    Ok(Response::json(&res_data).with_status_code(201))
}

fn validate_req_order(req_order: &ReqOrder) -> Result<(), FailureResponse> {
    match req_order.type_ {
        ApiOrderType::Normal => {
            if req_order.amount_token == 0 || req_order.amount_coin == 0 {
                log::warn!("Received 0 amount order request {:?}", req_order);
                Err(FailureResponse::InvalidPayload)
            } else if req_order.token_name.is_none() {
                log::warn!("token_name is not specified : {:?}", req_order);
                Err(FailureResponse::InvalidPayload)
            } else {
                Ok(())
            }
        }
        ApiOrderType::Reward => Err(FailureResponse::InvalidPayload),
    }
}

fn add_order(
    open_market: &mut OpenMarket,
    user_id: &UserId,
    req_order: &ReqOrder,
) -> Result<Order, FailureResponse> {
    match req_order.type_ {
        ApiOrderType::Normal => {
            let new_order = open_market.try_add_normal_order(
                user_id,
                req_order.token_name.as_ref().unwrap(),
                &AmountToken::from(req_order.amount_token),
            )?;
            if new_order
                .amount_coin()
                .is_around_slip_range(&AmountCoin::from(req_order.amount_coin))
            {
                Ok(new_order.clone())
            } else {
                log::info!("Slip is detected");
                Err(FailureResponse::InvalidPayload)
            }
        }
        ApiOrderType::Reward => {
            panic!("Never happens");
        }
    }
}
