use crate::{
    app::{validate_bearer_header, FailureResponse},
    domain::{
        models::{
            market::{Market, MarketId, NormalOrder, TokenId},
            num::{AmountCoin, AmountToken},
        },
        services::{
            market_store::{MarketStore, UpdateMarketLastOrderErrorKind},
            AccessTokenStore,
        },
    },
};
use rouille::{input::json::json_input, Request, Response};

pub fn post<S>(
    mut store: S,
    req: &Request,
    market_id: MarketId,
) -> Result<Response, FailureResponse>
where
    S: AccessTokenStore + MarketStore,
{
    let req_data = json_input::<ReqData>(&req).map_err(|_| FailureResponse::InvalidPayload)?;
    if req_data.amount_token == AmountToken(0) || req_data.amount_coin == AmountCoin(0) {
        return Err(FailureResponse::InvalidPayload);
    }

    // 認証チェック
    let user_id = validate_bearer_header(&mut store, req)?.user_id;

    let req_order = NormalOrder::new(
        user_id,
        req_data.token_id,
        req_data.amount_token,
        req_data.amount_coin,
    );

    let res = try_add_order(&mut store, market_id, req_order);
    store.commit()?;
    res
}

fn try_add_order<S>(
    store: &mut S,
    market_id: MarketId,
    req_order: NormalOrder,
) -> Result<Response, FailureResponse>
where
    S: MarketStore,
{
    // marketがopenかチェック
    let mut market = match store.query_market(&market_id)? {
        Some(Market::Open(m)) => m,
        _ => return Err(FailureResponse::ResourceNotFound),
    };

    market
        .try_order(req_order)
        // TODO : return more information about failure.
        .map_err(|_e| FailureResponse::InvalidPayload)?;

    // Save a new market
    match store.update_market_last_order(&market) {
        Ok(()) => {
            let (_id, new_order) = market.last_normal_order().unwrap();
            let res_data = ResData {
                token_id: new_order.token_id,
                amount_token: new_order.amount_token,
                amount_coin: new_order.amount_coin,
            };
            Ok(Response::json(&res_data).with_status_code(201))
        }
        // Retry when conflict
        Err(UpdateMarketLastOrderErrorKind::Conflict) => try_add_order(store, market_id, req_order),
        Err(UpdateMarketLastOrderErrorKind::NotOpen) => Err(FailureResponse::ResourceNotFound),
        Err(UpdateMarketLastOrderErrorKind::Error(e)) => Err(FailureResponse::from(e)),
    }
}

#[derive(Debug, Deserialize)]
struct ReqData {
    token_id: TokenId,
    amount_token: AmountToken,
    amount_coin: AmountCoin,
}

#[derive(Debug, Serialize)]
struct ResData {
    token_id: TokenId,
    amount_token: AmountToken,
    amount_coin: AmountCoin,
}
