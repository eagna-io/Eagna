use crate::{
    app::{validate_bearer_header, FailureResponse},
    domain::{
        models::{
            market::{Market, MarketId, NormalOrder, TokenId},
            num::{AmountCoin, AmountToken},
        },
        services::{market_store::MarketStore, AccessTokenStore},
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
    let req_data = json_input::<ReqData>(req).map_err(|_| FailureResponse::InvalidPayload)?;
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

    let open_market = {
        let mut locked_store = store.lock_market(&market_id)?;

        let mut open_market = match locked_store.query_market(&market_id)? {
            Some(Market::Open(m)) => m,
            _ => return Err(FailureResponse::ResourceNotFound),
        };

        open_market
            .try_order(req_order)
            // TODO : return more information about failure.
            .map_err(|_e| FailureResponse::InvalidPayload)?;

        locked_store.update_market_last_order(&open_market)?;

        open_market
    };

    store.commit()?;

    let (_id, new_order) = open_market.last_normal_order().unwrap();
    let res_data = ResData {
        token_id: new_order.token_id,
        amount_token: new_order.amount_token,
        amount_coin: new_order.amount_coin,
    };
    Ok(Response::json(&res_data).with_status_code(201))
}

#[derive(Debug, Deserialize)]
struct ReqData {
    token_id: TokenId,
    amount_token: AmountToken,
    amount_coin: AmountCoin,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResData {
    token_id: TokenId,
    amount_token: AmountToken,
    amount_coin: AmountCoin,
}
