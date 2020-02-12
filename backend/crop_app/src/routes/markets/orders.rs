mod get_list;
mod post;

pub use get_list::get_list;
pub use post::post;

use crop_domain::market::order::Order;
use crop_primitive::NonEmptyString;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReqOrder {
    token_name: NonEmptyString,
    amount_token: i32,
    amount_coin: i32,
    time: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResOrder<'a> {
    token_name: &'a str,
    amount_token: i32,
    amount_coin: i32,
    time: &'a DateTime<Utc>,
}

impl<'a> From<&'a Order> for ResOrder<'a> {
    fn from(order: &'a Order) -> ResOrder<'a> {
        ResOrder {
            token_name: order.token_name().as_str(),
            amount_token: order.amount_token().as_i32(),
            amount_coin: order.amount_coin().as_i32(),
            time: order.time(),
        }
    }
}
