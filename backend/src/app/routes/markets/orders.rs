mod get_list;
mod post;

pub use get_list::get_list;
pub use post::post;

use crate::domain::market::{NormalOrder, Order, OrderType};
use crate::primitive::NonEmptyString;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReqOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    token_name: Option<NonEmptyString>,
    amount_token: i32,
    amount_coin: i32,
    time: DateTime<Utc>,
    #[serde(rename = "type")]
    type_: ApiOrderType,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResOrder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    token_name: Option<&'a str>,
    amount_token: i32,
    amount_coin: i32,
    time: &'a DateTime<Utc>,
    #[serde(rename = "type")]
    type_: ApiOrderType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum ApiOrderType {
    CoinSupply,
    Normal,
    Reward,
}

impl<'a> From<&'a Order> for ResOrder<'a> {
    fn from(order: &'a Order) -> ResOrder<'a> {
        ResOrder {
            token_name: order.token_name().map(|name| name.as_str()),
            amount_token: order.amount_token().as_i32(),
            amount_coin: order.amount_coin().as_i32(),
            time: order.time(),
            type_: ApiOrderType::from(order.type_()),
        }
    }
}

impl<'a> From<&'a NormalOrder> for ResOrder<'a> {
    fn from(order: &'a NormalOrder) -> ResOrder<'a> {
        ResOrder {
            token_name: Some(order.token_name().as_str()),
            amount_token: order.amount_token().as_i32(),
            amount_coin: order.amount_coin().as_i32(),
            time: order.time(),
            type_: ApiOrderType::Normal,
        }
    }
}

impl From<OrderType> for ApiOrderType {
    fn from(order: OrderType) -> ApiOrderType {
        match order {
            OrderType::CoinSupply => ApiOrderType::CoinSupply,
            OrderType::Normal => ApiOrderType::Normal,
            OrderType::Reward => ApiOrderType::Reward,
        }
    }
}
