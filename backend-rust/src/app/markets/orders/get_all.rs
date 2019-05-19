use crate::{
    app::{validate_bearer_header, FailureResponse},
    domain::models::{
        market::{MarketId, NormalOrder, Order, OrderId, TokenId},
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
            let order_ids = orders.related_to_user(user_id).map(|(id, _o)| id).collect();
            Some(Mine { orders: order_ids })
        }
        _ => None,
    };

    let resp_orders = orders
        .iter()
        .filter_map(|(id, order)| match order {
            Order::Normal(o) => Some(RespOrder::from((id, *o))),
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
    orders: Vec<RespOrder>,
    mine: Option<Mine>,
}

#[derive(Debug, Serialize)]
struct RespOrder {
    id: OrderId,
    token_id: TokenId,
    amount_token: AmountToken,
    amount_coin: AmountCoin,
    time: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
struct Mine {
    orders: Vec<OrderId>,
}

impl From<(OrderId, NormalOrder)> for RespOrder {
    fn from(items: (OrderId, NormalOrder)) -> RespOrder {
        let (id, order) = items;
        RespOrder {
            id,
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
