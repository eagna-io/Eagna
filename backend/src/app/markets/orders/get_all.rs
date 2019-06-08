use crate::{
    app::{validate_bearer_header, FailureResponse},
    domain::models::{
        market::{MarketId, NormalOrder, Order, TokenId, OrderType},
        num::{AmountCoin, AmountToken},
    },
    domain::services::{AccessTokenStore, MarketStore},
};
use chrono::{DateTime, Utc};
use rouille::{Request, Response};

pub fn get_all<S>(
    mut store: S,
    req: &Request,
    market_id: MarketId,
) -> Result<Response, FailureResponse>
where
    S: AccessTokenStore + MarketStore,
{
    let market = match store.query_market(&market_id)? {
        Some(m) => m,
        None => return Err(FailureResponse::ResourceNotFound),
    };

    let orders = match market.orders() {
        Some(orders) => orders,
        None => return Ok(Response::json(&RespBody::empty())),
    };

    let maybe_mine = match req.get_param("contains") {
        Some(ref s) if s.as_str() == "mine" => {
            let user_id = validate_bearer_header(&mut store, req)?.user_id;
            let my_orders = orders
                .related_to_user(user_id)
                .map(|(_, order)| RespMyOrder {
                    token_id: order.token_id().cloned(),
                    amount_token: order.amount_token(),
                    amount_coin: order.amount_coin(),
                    time: *order.time(),
                    type_: order.type_(),
                })
                .collect();
            Some(Mine { orders: my_orders })
        }
        _ => None,
    };

    let resp_orders = orders
        .iter()
        .filter_map(|(_, order)| match order {
            Order::Normal(o) => Some(RespNormalOrder::from(*o)),
            _ => None,
        })
        .collect();

    let resp = RespBody {
        orders: resp_orders,
        mine: maybe_mine,
    };
    Ok(Response::json(&resp))
}

#[derive(Debug, Serialize)]
struct RespBody {
    orders: Vec<RespNormalOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mine: Option<Mine>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RespNormalOrder {
    token_id: TokenId,
    amount_token: AmountToken,
    amount_coin: AmountCoin,
    time: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
struct Mine {
    orders: Vec<RespMyOrder>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RespMyOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    token_id: Option<TokenId>,
    amount_token: AmountToken,
    amount_coin: AmountCoin,
    time: DateTime<Utc>,
    #[serde(rename = "type")]
    type_: OrderType,
}

impl From<NormalOrder> for RespNormalOrder {
    fn from(order: NormalOrder) -> RespNormalOrder {
        RespNormalOrder {
            token_id: order.token_id,
            amount_token: order.amount_token,
            amount_coin: order.amount_coin,
            time: order.time,
        }
    }
}

impl RespBody {
    fn empty() -> RespBody {
        RespBody {
            orders: Vec::new(),
            mine: None,
        }
    }
}
