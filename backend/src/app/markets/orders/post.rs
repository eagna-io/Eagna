use super::ApiOrderModel;
use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::{market::*, user::*};
use crate::infra::postgres::transaction;
use rouille::{input::json::json_input, Request, Response};

pub fn post(
    infra: &InfraManager,
    req: &Request,
    market_id: MarketId,
) -> Result<Response, FailureResponse> {
    let req_data = json_input::<ApiOrderModel>(req).map_err(|_| FailureResponse::InvalidPayload)?;

    validate_req_order(&req_data)?;

    let user_id = validate_bearer_header(infra, req)?.user_id;

    let postgres = infra.get_postgres()?;
    let added_order = transaction(postgres, || {
        let market_repo = MarketRepository::from(postgres);

        market_repo.lock_market(&market_id)?;

        let mut open_market = match market_repo.query_market(&market_id)? {
            Some(Market::Open(m)) => m,
            _ => return Err(FailureResponse::ResourceNotFound),
        };

        let added_order = add_order(&mut open_market, &user_id, &req_data)?;

        market_repo.save_market(&Market::from(open_market))?;

        Ok(added_order)
    })?;

    let res_data = ApiOrderModel::from(added_order);

    Ok(Response::json(&res_data).with_status_code(201))
}

fn validate_req_order(req_order: &ApiOrderModel) -> Result<(), FailureResponse> {
    match req_order.type_ {
        OrderType::CoinSupply => Ok(()),
        OrderType::Normal => {
            if req_order.amount_token == AmountToken::zero()
                || req_order.amount_coin == AmountCoin::zero()
            {
                log::warn!("Received 0 amount order request {:?}", req_order);
                Err(FailureResponse::InvalidPayload)
            } else if req_order.token_name.is_none() {
                log::warn!("token_name is not specified : {:?}", req_order);
                Err(FailureResponse::InvalidPayload)
            } else {
                Ok(())
            }
        }
        OrderType::Reward => Err(FailureResponse::InvalidPayload),
    }
}

fn add_order(
    open_market: &mut OpenMarket,
    user_id: &UserId,
    req_order: &ApiOrderModel,
) -> Result<Order, FailureResponse> {
    match req_order.type_ {
        OrderType::CoinSupply => Ok(open_market.try_supply_initial_coin(user_id)?.clone()),
        OrderType::Normal => {
            let new_order = open_market.try_add_normal_order(
                user_id,
                req_order.token_name.as_ref().unwrap(),
                &req_order.amount_token,
            )?;
            if new_order
                .amount_coin()
                .is_around_slip_range(&req_order.amount_coin)
            {
                Ok(new_order.clone())
            } else {
                log::info!("Slip is detected");
                Err(FailureResponse::InvalidPayload)
            }
        }
        OrderType::Reward => {
            panic!("Never happens");
        }
    }
}
