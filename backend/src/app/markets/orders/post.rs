use crate::{
    app::{validate_bearer_header, FailureResponse},
    domain::{
        models::{
            market::{Market, MarketId, NormalOrder, OpenMarket, Order, TokenId},
            num::{AmountCoin, AmountToken},
            user::UserId,
        },
        services::{market_store::MarketStore, AccessTokenStore},
    },
};
use chrono::{DateTime, Utc};
use rouille::{input::json::json_input, Request, Response};

pub fn post<S>(
    store: &mut S,
    req: &Request,
    market_id: MarketId,
) -> Result<Response, FailureResponse>
where
    S: AccessTokenStore + MarketStore,
{
    let req_data = json_input::<ReqData>(req).map_err(|_| FailureResponse::InvalidPayload)?;

    validate_req_data(&req_data)?;

    let user_id = validate_bearer_header(store, req)?.user_id;

    let mut locked_store = store.lock_market(&market_id)?;

    let mut open_market = match locked_store.query_market(&market_id)? {
        Some(Market::Open(m)) => m,
        _ => return Err(FailureResponse::ResourceNotFound),
    };

    let added_order = add_order(&mut open_market, &user_id, &req_data)?;
    locked_store.update_market_last_order(&open_market)?;

    let res_data = to_res_data(added_order);
    Ok(Response::json(&res_data).with_status_code(201))
}

fn validate_req_data(req_data: &ReqData) -> Result<(), FailureResponse> {
    match req_data {
        ReqData::InitialSupply => Ok(()),
        ReqData::Normal {
            amount_token,
            amount_coin,
            ..
        } => {
            if *amount_token == AmountToken(0) || *amount_coin == AmountCoin(0) {
                log::warn!("Received 0 amount order request {:?}", req_data);
                Err(FailureResponse::InvalidPayload)
            } else {
                Ok(())
            }
        }
    }
}

fn add_order(
    open_market: &mut OpenMarket,
    user_id: &UserId,
    req_data: &ReqData,
) -> Result<Order, FailureResponse> {
    match req_data {
        ReqData::InitialSupply => open_market
            .try_supply_initial_coin(user_id)
            .map_err(|e| {
                log::warn!("Failed to supply initial coin to {:?} : {:?}", user_id, e);
                FailureResponse::InvalidPayload
            })
            .map(|o| Order::InitialSupply(o)),
        ReqData::Normal {
            token_id,
            amount_token,
            amount_coin,
        } => {
            let req_order = NormalOrder::new(*user_id, *token_id, *amount_token, *amount_coin);
            open_market
                .try_order(req_order)
                // TODO : return more information about failure.
                .map_err(|e| {
                    log::info!("Failed to apply a new order : {:?}", e);
                    FailureResponse::InvalidPayload
                })
                .map(|o| Order::Normal(o))
        }
    }
}

fn to_res_data(order: Order) -> ResData {
    match order {
        Order::InitialSupply(o) => ResData::InitialSupply {
            amount_coin: o.amount_coin,
            time: o.time,
        },
        Order::Normal(o) => ResData::Normal {
            token_id: o.token_id,
            amount_token: o.amount_token,
            amount_coin: o.amount_coin,
            time: o.time,
        },
        Order::Settle(_) => panic!("Never happens"),
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum ReqData {
    InitialSupply,
    Normal {
        token_id: TokenId,
        amount_token: AmountToken,
        amount_coin: AmountCoin,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum ResData {
    InitialSupply {
        amount_coin: AmountCoin,
        time: DateTime<Utc>,
    },
    Normal {
        token_id: TokenId,
        amount_token: AmountToken,
        amount_coin: AmountCoin,
        time: DateTime<Utc>,
    },
}
