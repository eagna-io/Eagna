use crate::{
    app::{validate_bearer_header, FailureResponse},
    domain::{
        models::market::{Market, MarketId, NormalOrder, TokenId},
        services::{
            market_store::{MarketStore, UpdateMarketLastOrderResult},
            AccessTokenStore,
        },
    },
};
use rouille::{input::json::json_input, Request, Response};

pub fn post<S>(store: &S, req: &Request, market_id: MarketId) -> Result<Response, FailureResponse>
where
    S: AccessTokenStore + MarketStore,
{
    let req_data = json_input::<ReqData>(&req).map_err(|_| FailureResponse::InvalidPayload)?;
    if req_data.amount_token == 0 || req_data.amount_coin == 0 {
        return Err(FailureResponse::InvalidPayload);
    }

    // 認証チェック
    let user_id = validate_bearer_header(store, req)?.user_id;

    let req_order = NormalOrder::new(
        user_id,
        req_data.token_id,
        req_data.amount_token,
        req_data.amount_coin,
    );

    try_add_order(store, market_id, req_order)
}

fn try_add_order<S>(
    store: &S,
    market_id: MarketId,
    req_order: NormalOrder,
) -> Result<Response, FailureResponse>
where
    S: MarketStore,
{
    // marketがopenかチェック
    let mut market = match store.query_market(&market_id) {
        Ok(Some(Market::Open(m))) => m,
        Ok(_) => return Err(FailureResponse::ResourceNotFound),
        Err(e) => {
            dbg!(e);
            return Err(FailureResponse::ServerError);
        }
    };

    market
        .try_order(req_order)
        // TODO : return more information about failure.
        .map_err(|_e| FailureResponse::InvalidPayload)?;

    // Save a new market
    match store.update_market_last_order(&market) {
        UpdateMarketLastOrderResult::Success => {
            let new_order = *market.last_normal_order().unwrap();
            let res_data = ResData {
                token_id: new_order.token_id,
                amount_token: new_order.amount_token,
                amount_coin: new_order.amount_coin,
            };
            Ok(Response::json(&res_data).with_status_code(201))
        }
        // Retry when conflict
        UpdateMarketLastOrderResult::Conflict => try_add_order(store, market_id, req_order),
        UpdateMarketLastOrderResult::NotOpen => Err(FailureResponse::ResourceNotFound),
        UpdateMarketLastOrderResult::Error(e) => {
            dbg!(e);
            Err(FailureResponse::ServerError)
        }
    }
}

#[derive(Debug, Deserialize)]
struct ReqData {
    token_id: TokenId,
    amount_token: i32,
    amount_coin: i32,
}

#[derive(Debug, Serialize)]
struct ResData {
    token_id: TokenId,
    amount_token: i32,
    amount_coin: i32,
}
